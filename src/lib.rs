use std::io::Read;
use std::{fs::OpenOptions, path::Path};

use anyhow::Error;
use anyhow::{anyhow, Context};
use instructions::add::parse_add;
use instructions::and::parse_and;
use instructions::call::parse_call;
use instructions::clear::parse_clear;
use instructions::cmp::parse_cmp;
use instructions::dec::parse_dec;
use instructions::halt::parse_halt;
use instructions::inc::parse_inc;
use instructions::jump::{parse_jer, parse_jof, parse_jok, parse_jump, parse_jze};
use instructions::noop::parse_noop;
use instructions::not::parse_not;
use instructions::or::parse_or;
use instructions::peek::parse_peek;
use instructions::pop::parse_pop;
use instructions::push::parse_push;
use instructions::return_::parse_return;
use instructions::set::parse_set;
use instructions::shift::parse_shift;
use instructions::store::parse_store;
use instructions::sub::parse_sub;
use instructions::swap::parse_swap;
use instructions::xor::parse_xor;
use instructions::zero::parse_zero;
use opcodes::Opcode;

mod instructions;
mod opcodes;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Instruction,      // any of instructions string
    Register,         // any of A, B, HI, LI, AB, HLI, EX, IRA. Also S, SA, SS for easier parsing
    Flag,             // any of ERR, IRQ, OK, OVF, ZERO
    ImmediateValue8,  // any token starting with `#0x` and parsing into 1 byte value
    ImmediateValue16, // any token starting with `#0x` and parsing into 2 byte value
    Indirection,      // `&HLI`
    Address,          // any 2 byte token starting with `&0x`
    Label,            // any text ending with ":"
    Text,             // any token that does not match the rest
    AddressDelimiter, // `>`
    CommentStart,     // `//`
    DataStream,       // `$`
}

#[derive(Debug, Clone)]
struct Token {
    _type: TokenType,
    raw: String,
    opcode: Option<Opcode>,
    value: Option<usize>, // since the value can be either u8 or u16
    address: Option<u16>,
}

impl Token {
    pub fn formatted_raw(&self) -> String {
        self.raw.trim().to_uppercase()
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            _type: TokenType::Text,
            raw: Default::default(),
            opcode: Default::default(),
            value: Default::default(),
            address: Default::default(),
        }
    }
}

impl TryFrom<String> for Token {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "A" | "B" | "HI" | "LI" | "AB" | "HLI" | "EX" | "IRA" | "S" | "SA" | "SS" => {
                Ok(Token {
                    _type: TokenType::Register,
                    raw: value,
                    ..Default::default()
                })
            }
            immediate if immediate.starts_with("0X") => {
                let parsed_val =
                    usize::from_str_radix(&immediate[2..], 16).map_err(|e| anyhow!(e))?;
                let token_type = if immediate.len() > 5 {
                    TokenType::ImmediateValue16
                } else {
                    TokenType::ImmediateValue8
                };

                Ok(Token {
                    _type: token_type,
                    raw: value,
                    value: Some(parsed_val),
                    ..Default::default()
                })
            }
            label_value if label_value.starts_with('*') => Ok(Token {
                _type: TokenType::ImmediateValue16,
                raw: value,
                ..Default::default()
            }),
            indirection if indirection == "&HLI" => Ok(Token {
                _type: TokenType::Indirection,
                raw: value,
                ..Default::default()
            }),
            address if address.starts_with('&') => {
                let parsed_val =
                    usize::from_str_radix(&address[3..], 16).map_err(|e| anyhow!(e))?;
                Ok(Token {
                    _type: TokenType::Address,
                    raw: value,
                    value: Some(parsed_val),
                    ..Default::default()
                })
            }
            "$" => Ok(Token {
                _type: TokenType::DataStream,
                raw: value,
                ..Default::default()
            }),
            ">" => Ok(Token {
                _type: TokenType::AddressDelimiter,
                raw: value,
                ..Default::default()
            }),
            "//" => Ok(Token {
                _type: TokenType::CommentStart,
                raw: value,
                ..Default::default()
            }),
            "ERR" | "IRQ" | "OK" | "OVF" | "ZER" => Ok(Token {
                _type: TokenType::Flag,
                raw: value,
                ..Default::default()
            }),
            "NOOP" | "PUSH" | "POP" | "PEEK" | "STO" | "ADD" | "SUB" | "SHL" | "SHR" | "AND"
            | "OR" | "XOR" | "NOT" | "CMP" | "INC" | "DEC" | "ZERO" | "SWP" | "JZE" | "JOF"
            | "JER" | "JOK" | "JMP" | "CALL" | "RET" | "SET" | "CLR" | "HALT" => Ok(Token {
                _type: TokenType::Instruction,
                raw: value,
                ..Default::default()
            }),
            label if label.ends_with(':') => Ok(Token {
                _type: TokenType::Label,
                raw: value,
                ..Default::default()
            }),
            _ => Ok(Token {
                _type: TokenType::Text,
                raw: value,
                ..Default::default()
            }),
        }
    }
}

pub struct Assembler<'a> {
    input_path: &'a Path,
    input: String,
    tokens: Vec<Vec<Token>>,
    parsed_tokens: Vec<Token>,
}

impl<'a> Assembler<'a> {
    pub fn new(input_path: &'a Path) -> Self {
        Self {
            input_path,
            input: String::new(),
            tokens: vec![],
            parsed_tokens: vec![],
        }
    }

    pub fn assemble(&mut self, verbose: bool) -> Result<Vec<u8>, Error> {
        self.load_input()?;
        self.parse_tokens(verbose)?;
        self.generate_bytes(verbose)
    }

    fn load_input(&mut self) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(self.input_path)
            .map_err(|e| anyhow!(e))?;
        file.read_to_string(&mut self.input)
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    fn parse_tokens(&mut self, verbose: bool) -> Result<(), Error> {
        // First pass - convert text to Token structs
        self.tokens = self
            .input
            .split('\n')
            .enumerate()
            .map(|(line_n, line)| {
                let mut comment = false;
                line.split_ascii_whitespace()
                    .enumerate()
                    .map(|(word_n, word)| {
                        if !comment {
                            let token = Token::try_from(word.to_string())
                                .map_err(|e| {
                                    anyhow!(
                                        "Error: {} in line {} token {}: {}",
                                        e,
                                        line_n + 1,
                                        word_n + 1,
                                        word
                                    )
                                })
                                .unwrap();
                            comment = token._type == TokenType::CommentStart;
                            if verbose {
                                println!("Parsed token: {:?}", token);
                            }
                            token
                        } else {
                            Token {
                                _type: TokenType::Text,
                                raw: word.to_owned(),
                                opcode: None,
                                value: None,
                                address: None,
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        // Second pass - assign opcodes and remove unnecessary tokens, flatten the structure
        let mut current_mem_address: u16 = 0x0000;
        self.parsed_tokens = self
            .tokens
            .iter()
            .enumerate()
            .filter_map(|(line_n, line)| {
                // First token on each line can only be Instruction, Label, Comment, DataStream or AddressDelimiter
                if let Some(first_token) = line.get(0) {
                    match first_token._type {
                        TokenType::Instruction => {
                            let parsed = Some(Self::parse_instruction(line, &mut current_mem_address).context(anyhow!("error in line {}", line_n + 1)));
                            current_mem_address += 1;
                            parsed
                        }
                        TokenType::Label => {
                            let mut label = first_token.clone();
                            label.address = Some(current_mem_address);
                            Some(Ok(vec![label]))
                        }
                        TokenType::AddressDelimiter => {
                            // This changes `current_mem_address`
                            if let Some(address_token) = line.get(1) {
                                current_mem_address = address_token.value.unwrap() as u16;
                                None
                            } else {
                                Some(Err(anyhow!(
                                    "Syntax error in line {} - cannot read address after address delimiter",
                                    line_n + 1,
                                )))
                            }
                            
                        }
                        TokenType::CommentStart => {
                            // This line is a comment, ignore it
                            None
                        }
                        TokenType::DataStream => {
                            let mut parsed_data_stream = vec![];
                            let mut started_text = false;
                            for token in line.iter().skip(1) {
                                let mut token_clone = token.clone();  
                                match token._type {
                                    TokenType::Text => {
                                        started_text = token_clone.raw.starts_with('\"') & !started_text & !token_clone.raw.ends_with('\"');
                                        token_clone.raw = token.raw.replace('\"', "");
                                        for char in token_clone.raw.chars() {
                                            let char_token = Token {
                                                _type: TokenType::ImmediateValue8,
                                                raw: char.to_string(),
                                                value: Some(char as u8 as usize),
                                                address: Some(current_mem_address),
                                                opcode: None
                                            };
                                            current_mem_address += 1;
                                            parsed_data_stream.push(char_token);
                                        }
                                        if started_text {
                                            parsed_data_stream.push(Token {
                                                _type: TokenType::ImmediateValue8,
                                                raw: " ".to_string(),
                                                value: Some(b' ' as usize),
                                                address: Some(current_mem_address),
                                                opcode: None 
                                            });
                                        }
                                    },
                                    TokenType::ImmediateValue8 => {
                                        token_clone.address = Some(current_mem_address);
                                        token_clone._type = TokenType::ImmediateValue8;
                                        token_clone.value = token.value;
                                        parsed_data_stream.push(token_clone);
                                    },
                                    TokenType::ImmediateValue16 => {
                                        token_clone.address = Some(current_mem_address);
                                        token_clone._type = TokenType::ImmediateValue16;
                                        token_clone.value = token.value;
                                        parsed_data_stream.push(token_clone);
                                    },
                                    _ => {
                                        return Some(Err(anyhow!("Syntax error in line {} - invalid token after $: {}", line_n + 1, token.raw)))
                                    }
                                }
                                current_mem_address += 1;
                            }
                            Some(Ok(parsed_data_stream))
                    },
                        _ => Some(Err(anyhow!(
                            "Syntax error in line {} - line cannot start with {}",
                            line_n + 1,
                            first_token.raw
                        ))),
                    }
                } else {
                    None
                }
            }).flat_map(|line| {
                if verbose {
                    println!("Parsed line: {:?}", line); 
                }
                line.unwrap()
            })
            .collect();

        // Third pass: assign values for `Text` or `ImmediateValue16` tokens that are labels
        let pt_clone = self.parsed_tokens.clone();
        for token in &mut self.parsed_tokens {
            match token._type {
                TokenType::Text => {
                    let mut labels = pt_clone.iter().filter(|t| t._type == TokenType::Label);
                    let mut token_val = token.raw.clone();
                    token_val.push(':');
                    if let Some(label) = labels.find(|t| t.raw == token_val) {
                        token._type = TokenType::Label;
                        token.value = Some(label.address.unwrap() as usize);
                    }
                }
                TokenType::ImmediateValue16 => {
                    let mut labels = pt_clone.iter().filter(|t| t._type == TokenType::Label);
                    let mut token_val = token.raw.clone();
                    token_val.push(':');
                    if let Some(label_name) = token_val.strip_prefix('*') {
                        if let Some(label) = labels.find(|t| t.raw == label_name) {
                            token._type = TokenType::Label;
                            token.value = Some(label.address.unwrap() as usize);
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(())
    }

    fn parse_instruction(
        tokenised_line: &Vec<Token>,
        current_mem_address: &mut u16,
    ) -> Result<Vec<Token>, Error> {
        match tokenised_line.get(0).unwrap().formatted_raw().as_str() {
            "NOOP" => parse_noop(tokenised_line, current_mem_address),
            "PUSH" => parse_push(tokenised_line, current_mem_address),
            "POP" => parse_pop(tokenised_line, current_mem_address),
            "PEEK" => parse_peek(tokenised_line, current_mem_address),
            "STO" => parse_store(tokenised_line, current_mem_address),
            "ADD" => parse_add(tokenised_line, current_mem_address),
            "SUB" => parse_sub(tokenised_line, current_mem_address),
            "SHL" => parse_shift(
                tokenised_line,
                current_mem_address,
                instructions::shift::Direction::Left,
            ),
            "SHR" => parse_shift(
                tokenised_line,
                current_mem_address,
                instructions::shift::Direction::Right,
            ),
            "AND" => parse_and(tokenised_line, current_mem_address),
            "OR" => parse_or(tokenised_line, current_mem_address),
            "XOR" => parse_xor(tokenised_line, current_mem_address),
            "NOT" => parse_not(tokenised_line, current_mem_address),
            "CMP" => parse_cmp(tokenised_line, current_mem_address),
            "INC" => parse_inc(tokenised_line, current_mem_address),
            "DEC" => parse_dec(tokenised_line, current_mem_address),
            "ZERO" => parse_zero(tokenised_line, current_mem_address),
            "SWP" => parse_swap(tokenised_line, current_mem_address),
            "JZE" => parse_jze(tokenised_line, current_mem_address),
            "JOF" => parse_jof(tokenised_line, current_mem_address),
            "JER" => parse_jer(tokenised_line, current_mem_address),
            "JOK" => parse_jok(tokenised_line, current_mem_address),
            "JMP" => parse_jump(tokenised_line, current_mem_address),
            "CALL" => parse_call(tokenised_line, current_mem_address),
            "RET" => parse_return(tokenised_line, current_mem_address),
            "SET" => parse_set(tokenised_line, current_mem_address),
            "CLR" => parse_clear(tokenised_line, current_mem_address),
            "HALT" => parse_halt(tokenised_line, current_mem_address),
            val => Err(anyhow!(
                "{} parsed as a instruction despite not being one!",
                val
            )),
        }
    }

    fn generate_bytes(&self, verbose: bool) -> Result<Vec<u8>, Error> {
        let mut result = vec![0; 0xffff + 1];
        for token in &self.parsed_tokens {
            let address = token.address.unwrap() as usize;
            if token.opcode.is_some() {
                let instruction_val = token.opcode.clone().unwrap() as u8;
                if verbose {
                    println!(
                        "Writing instruction: {:02x?} (0x{:02x})",
                        token, instruction_val
                    );
                }
                result[address] = instruction_val;
            } else if token.value.is_some() {
                if token._type == TokenType::ImmediateValue8 {
                    if verbose {
                        println!("Writing 8 bit data: {:02x?}", token);
                    }
                    result[address] = (token.value.unwrap() & 0xff) as u8;
                } else if token._type == TokenType::ImmediateValue16
                    || token._type == TokenType::Label
                    || token._type == TokenType::Address
                {
                    if verbose {
                        println!("Writing 16 bit data: {:04x?}", token);
                    }
                    result[address] = ((token.value.unwrap() & 0xff00) >> 8) as u8;
                    result[address + 1] = (token.value.unwrap() & 0xff) as u8;
                }
            }
        }
        Ok(result)
    }
}
