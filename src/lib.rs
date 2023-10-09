use std::io::Read;
use std::{fs::OpenOptions, path::Path};

use anyhow::anyhow;
use anyhow::Error;
use instructions::add::parse_add;
use instructions::and::parse_and;
use instructions::call::parse_call;
use instructions::cmp::parse_cmp;
use instructions::dec::parse_dec;
use instructions::halt::parse_halt;
use instructions::inc::parse_inc;
use instructions::jump::{parse_jze, parse_jof, parse_jer, parse_jok, parse_jump};
use instructions::noop::parse_noop;
use instructions::not::parse_not;
use instructions::or::parse_or;
use instructions::pop::parse_pop;
use instructions::push::parse_push;
use instructions::shift::parse_shift;
use instructions::store::parse_sto;
use instructions::sub::parse_sub;
use instructions::swap::parse_swap;
use instructions::xor::parse_xor;
use instructions::zero::parse_zero;
use opcodes::Opcode;

mod opcodes;
mod instructions;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Instruction,      // any of instructions string
    Register,         // any of A, B, HI, LI, AB, HLI, EX, IRA. Also S, SA, SS for easier parsing
    Flag,             // any of ERR, IRQ, OK, OVF, ZERO
    ImmediateValue8,   // any token starting with `#0x` and parsing into 1 byte value
    ImmediateValue16,   // any token starting with `#0x` and parsing into 2 byte value
    Address,          // any 2 byte token starting with `0x`
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

impl TryFrom<String> for Token {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "A" | "B" | "HI" | "LI" | "AB" | "HLI" | "EX" | "IRA" | "S" | "SA" | "SS" => {
                Ok(Token {
                    _type: TokenType::Register,
                    raw: value,
                    opcode: None,
                    value: None,
                    address: None,
                })
            }
            im if im.starts_with("#0X") => {
                let parsed_val = usize::from_str_radix(&im[3..], 16).map_err(|e| anyhow!(e))?;
                let token_type = if im.len() > 5 {
                    TokenType::ImmediateValue16
                } else {
                    TokenType::ImmediateValue8
                };

                Ok(Token {
                    _type: token_type,
                    raw: value,
                    opcode: None,
                    value: Some(parsed_val),
                    address: None,
                })
            }
            addr if addr.starts_with("0X") => {
                let parsed_val = usize::from_str_radix(&addr[2..], 16).map_err(|e| anyhow!(e))?;
                Ok(Token {
                    _type: TokenType::Address,
                    raw: value,
                    opcode: None,
                    value: Some(parsed_val),
                    address: None,
                })
            }
            "$" => Ok(Token {
                _type: TokenType::DataStream,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            ">" => Ok(Token {
                _type: TokenType::AddressDelimiter,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            "//" => Ok(Token {
                _type: TokenType::CommentStart,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            "ERR" | "IRQ" | "OK" | "OVF" | "ZER" => Ok(Token {
                _type: TokenType::Flag,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            "NOOP" | "PUSH" | "POP" | "STO" | "ADD" | "SUB" | "SHL" | "SHR" | "AND" | "OR"
            | "XOR" | "NOT" | "CMP" | "INC" | "DEC" | "ZERO" | "SWP" | "JZE" | "JOF" | "JER"
            | "JOK" | "JMP" | "CALL" | "RET" | "SET" | "CLR" | "HALT" => Ok(Token {
                _type: TokenType::Instruction,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            label if label.ends_with(':') => Ok(Token {
                _type: TokenType::Label,
                raw: value,
                opcode: None,
                value: None,
                address: None,
            }),
            _ => Ok(Token {
                _type: TokenType::Text,
                raw: value,
                opcode: None,
                value: None,
                address: None,
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

    pub fn assemble(&mut self) -> Result<Vec<u8>, Error> {
        self.load_input()?;
        self.parse_tokens()?;
        self.generate_bytes()
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

    fn parse_tokens(&mut self) -> Result<(), Error> {
        // First pass - convert text to Token structs
        self.tokens = self
            .input
            .split('\n')
            .into_iter()
            .enumerate()
            .map(|(line_n, line)| {
                let mut comment = false;
                line.split_ascii_whitespace()
                    .into_iter()
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
                        println!("Parsed token: {:?}", token);
                        token
                        } else {
                            Token { _type: TokenType::Text, raw: word.to_owned(), opcode: None, value: None, address: None }
                        }
                        
                    })
                    .collect()
            })
            .collect();

        // Second pass - assign opcodes and remove unnecessary tokens, flatten the structure
        let mut current_mem_address: u16 = 0xffff; // to compensate for the +1 on first token
        self.parsed_tokens = self
            .tokens
            .iter()
            .enumerate()
            .filter_map(|(line_n, line)| {
                // First token on each line can only be Instruction, Label, Comment, DataStream or AddressDelimiter
                if let Some(first_token) = line.get(0) {
                    current_mem_address += 1;
                    match first_token._type {
                        TokenType::Instruction => Some(Self::parse_instruction(line, &mut current_mem_address)),
                        TokenType::Label => {
                            let mut label = first_token.clone();
                            label.address = Some(current_mem_address);
                            current_mem_address -= 1; // compensate for +1 on next line
                            Some(Ok(vec![label]))
                        }
                        TokenType::AddressDelimiter => {
                            // This changes `current_mem_address`
                            if let Some(address_token) = line.get(1) {
                                current_mem_address = address_token.value.unwrap() as u16 - 1; // compensate for the +1 on next line
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
                        TokenType::DataStream => Some(
                            line.iter().enumerate()
                                .map(|(idx, token)| {
                                    let mut token_clone = token.clone();                                            
                                    token_clone.address = Some(current_mem_address - 1 + idx as u16); // -1 because of DataStream symbol
                                    match token._type {
                                        TokenType::Address => {
                                            // normal byte interpreted as address
                                            token_clone._type = TokenType::ImmediateValue8;
                                            token_clone.value = Some(usize::from_str_radix(&token.raw[2..], 16).map_err(|e| anyhow!(e))?);
                                            Ok(token_clone)
                                        },
                                        TokenType::Text => {
                                            token_clone.raw = token.raw.replace('"', "");
                                            Ok(token_clone)
                                        }
                                        TokenType::DataStream => {
                                            Ok(token_clone)
                                        }
                                        _ => {
                                            Err(anyhow!("Syntax error in line {} - invalid token after $: {}", line_n + 1, token.raw))
                                        }
                                    }
                                })
                                .collect(),
                        ),
                        _ => Some(Err(anyhow!(
                            "Syntax error in line {} - line cannot start with {}",
                            line_n + 1,
                            first_token.raw
                        ))),
                    }
                } else {
                    None
                }
            }).map(|line| {
                println!("Parsed line: {:?}", line);
                line.unwrap()
            })
            .flatten()
            .collect();

        // Third pass: assign values for `Text` tokens that are labels
        let pt_clone = self.parsed_tokens.clone();
        for token in &mut self.parsed_tokens {
            if token._type == TokenType::Text {
                let mut labels = pt_clone.iter().filter(|t|t._type == TokenType::Label);
                let mut token_val = token.raw.clone();
                token_val.push(':');
                if let Some(label) = labels.find(|t| t.raw == token_val) {
                    token._type = TokenType::Label;
                    token.value = Some(label.address.unwrap() as usize);
                }
            }
        }
        Ok(())
    }

    fn parse_instruction(tokenised_line: &Vec<Token>, current_mem_address: &mut u16) -> Result<Vec<Token>, Error> {
        match tokenised_line.get(0).unwrap().raw.to_uppercase().as_str() {
            "NOOP" => parse_noop(tokenised_line, current_mem_address),
            "PUSH" => parse_push(tokenised_line, current_mem_address),
            "POP" => parse_pop(tokenised_line, current_mem_address),
            "STO" => parse_sto(tokenised_line, current_mem_address),
            "ADD" => parse_add(tokenised_line, current_mem_address),
            "SUB" => parse_sub(tokenised_line, current_mem_address),
            "SHL" => parse_shift(tokenised_line, current_mem_address, instructions::shift::Direction::Left),
            "SHR" => parse_shift(tokenised_line, current_mem_address, instructions::shift::Direction::Right),
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
            "RET" => todo!(),
            "SET" => todo!(),
            "CLR" => todo!(),
            "HALT" => parse_halt(tokenised_line, current_mem_address),
            val => Err(anyhow!(
                "{} parsed as a instruction despite not being one!",
                val
            )),
        }
    }

    fn generate_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut result = vec![0; 0xffff + 1];
        for token in &self.parsed_tokens {
            let address = token.address.unwrap() as usize;
            if token.opcode.is_some() {
                let instruction_val = token.opcode.clone().unwrap() as u8;
                println!("Writing instruction: {:02x?} (0x{:02x})", token, instruction_val);
                result[address] = instruction_val;
            } else if token.value.is_some() {
                if token._type == TokenType::ImmediateValue8 {
                    println!("Writing 8 bit data: {:02x?}", token);
                    result[address] = (token.value.unwrap() & 0xff) as u8;
                } else if token._type == TokenType::ImmediateValue16 || token._type == TokenType::Label || token._type == TokenType::Address{
                    println!("Writing 16 bit data: {:04x?}", token);
                    result[address] = ((token.value.unwrap() & 0xff00) >> 8) as u8;
                    result[address + 1] = (token.value.unwrap() & 0xff) as u8;
                }
            }
        }
        Ok(result)
    }
}
