use std::io::Read;
use std::{fs::OpenOptions, path::Path};

use anyhow::anyhow;
use anyhow::Error;
use opcodes::Opcode;

mod opcodes;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Instruction,      // any of instructions string
    Register,         // any of A, B, HI, LI, AB, HLI, EX, IRA. Also S, SA, SS for easier parsing
    Flag,             // any of ERR, IRQ, OK, OVF, ZERO
    ImmediateValue,   // any token starting with `#0x` and parsing into 1 or 2 byte value
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
                })
            }
            im if im.starts_with("#0X") => {
                let parsed_val = usize::from_str_radix(&im[3..], 16).map_err(|e| anyhow!(e))?;
                Ok(Token {
                    _type: TokenType::ImmediateValue,
                    raw: value,
                    opcode: None,
                    value: Some(parsed_val),
                })
            }
            addr if addr.starts_with("0X") => {
                let parsed_val = usize::from_str_radix(&addr[2..], 16).map_err(|e| anyhow!(e))?;
                Ok(Token {
                    _type: TokenType::Address,
                    raw: value,
                    opcode: None,
                    value: Some(parsed_val),
                })
            }
            "$" => Ok(Token {
                _type: TokenType::DataStream,
                raw: value,
                opcode: None,
                value: None,
            }),
            ">" => Ok(Token {
                _type: TokenType::AddressDelimiter,
                raw: value,
                opcode: None,
                value: None,
            }),
            "//" => Ok(Token {
                _type: TokenType::CommentStart,
                raw: value,
                opcode: None,
                value: None,
            }),
            "ERR" | "IRQ" | "OK" | "OVF" | "ZER" => Ok(Token {
                _type: TokenType::Flag,
                raw: value,
                opcode: None,
                value: None,
            }),
            "NOOP" | "PUSH" | "POP" | "STO" | "ADD" | "SUB" | "SHL" | "SHR" | "AND" | "OR"
            | "XOR" | "NOT" | "CMP" | "INC" | "DEC" | "ZERO" | "SWP" | "JZE" | "JOF" | "JER"
            | "JOK" | "JMP" | "CALL" | "RET" | "SET" | "CLR" | "HALT" => Ok(Token {
                _type: TokenType::Instruction,
                raw: value,
                opcode: None,
                value: None,
            }),
            label if label.ends_with(':') => Ok(Token {
                _type: TokenType::Label,
                raw: value,
                opcode: None,
                value: None,
            }),
            _ => Ok(Token {
                _type: TokenType::Text,
                raw: value,
                opcode: None,
                value: None,
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
        println!("Parsed tokens: {:?}", self.tokens);
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
            .filter(|(_, line)| !line.is_empty())
            .map(|(line_n, line)| {
                line.split_ascii_whitespace()
                    .into_iter()
                    .enumerate()
                    .map(|(word_n, word)| {
                        Token::try_from(word.to_string())
                            .map_err(|e| {
                                anyhow!(
                                    "Error: {} in line {} token {}: {}",
                                    e,
                                    line_n,
                                    word_n,
                                    word
                                )
                            })
                            .unwrap()
                    })
                    .collect()
            })
            .collect();

        // Second pass
        self.parsed_tokens = self
            .tokens
            .iter()
            .enumerate()
            .filter_map(|(line_n, line)| {
                // First token on each line can only be Instruction, Label, Comment, DataStream or AddressDelimiter
                let first_token = line.get(0).unwrap();
                match first_token._type {
                    TokenType::Instruction => Some(Self::parse_instruction(line)),
                    TokenType::Label => todo!(),
                    TokenType::AddressDelimiter => todo!(),
                    TokenType::CommentStart => {
                        // This line is a comment, ignore it
                        None
                    }
                    TokenType::DataStream => todo!(),
                    _ => Some(Err(anyhow!(
                        "Syntax error in line {} - line cannot start with {}",
                        line_n,
                        first_token.raw
                    ))),
                }
            })
            .map(|token| token.unwrap())
            .collect();
        Ok(())
    }

    fn parse_instruction(tokenised_line: &Vec<Token>) -> Result<Token, Error> {
        let instruction = tokenised_line.get(0).unwrap().clone();
        match instruction.raw.as_str() {
            "NOOP" => todo!(),
            "PUSH" => todo!(),
            "POP" => todo!(),
            "STO" => todo!(),
            "ADD" => todo!(),
            "SUB" => todo!(),
            "SHL" => todo!(),
            "SHR" => todo!(),
            "AND" => todo!(),
            "OR" => todo!(),
            "XOR" => todo!(),
            "NOT" => todo!(),
            "CMP" => todo!(),
            "INC" => todo!(),
            "DEC" => todo!(),
            "ZERO" => todo!(),
            "SWP" => todo!(),
            "JZE" => todo!(),
            "JOF" => todo!(),
            "JER" => todo!(),
            "JOK" => todo!(),
            "JMP" => todo!(),
            "CALL" => todo!(),
            "RET" => todo!(),
            "SET" => todo!(),
            "CLR" => todo!(),
            "HALT" => todo!(),
            val => Err(anyhow!(
                "{} parsed as a instruction despite not being one!",
                val
            )),
        }
    }

    fn generate_bytes(&self) -> Result<Vec<u8>, Error> {
        todo!()
    }
}
