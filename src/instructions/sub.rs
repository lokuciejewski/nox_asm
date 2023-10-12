use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_sub(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut tokens = tokenised_line
        .get(0..=2)
        .ok_or_else(|| anyhow!("too few tokens in line for PUSH"))?
        .to_owned();

    match tokens.as_mut_slice() {
        [instruction, token_1, token_2] => {
            let mut instruction = instruction.clone();
            instruction.address = Some(*current_mem_address);
            match (&instruction._type, &token_1._type, &token_2._type) {
                (
                    TokenType::Instruction,
                    TokenType::Address | TokenType::Text | TokenType::Label,
                    TokenType::Register,
                ) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::SUB_ABSOLUTE_A,
                        "B" => Opcode::SUB_ABSOLUTE_B,
                        "AB" => Opcode::SUB_ABSOLUTE_AB,
                        _ => return Err(anyhow!("only A, B or AB can be used in SUB absolute")),
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Register) => {
                    instruction.opcode = Some(
                        match (
                            token_1.formatted_raw().as_str(),
                            token_2.formatted_raw().as_str(),
                        ) {
                            ("A", "B") => Opcode::SUB_A_B,
                            ("B", "A") => Opcode::SUB_B_A,
                            _ => return Err(anyhow!("only A and B can be substracted together")),
                        },
                    );
                    Ok(vec![instruction])
                }
                (TokenType::Instruction, TokenType::ImmediateValue8, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::SUB_IMMEDIATE_A,
                        "B" => Opcode::SUB_IMMEDIATE_B,
                        _ => {
                            return Err(anyhow!(
                                "only A and B can be used with immediate 8 bit value"
                            ))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::ImmediateValue16, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "AB" => Opcode::SUB_IMMEDIATE_AB,
                        _ => {
                            return Err(anyhow!("only AB can be used with immediate 16 bit value"))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => todo!(),
    }
}
