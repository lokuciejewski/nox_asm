use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_push(
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
                (TokenType::Instruction, TokenType::ImmediateValue8, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::PUSH_IMMEDIATE_A,
                        "B" => Opcode::PUSH_IMMEDIATE_B,
                        "HI" => Opcode::PUSH_IMMEDIATE_HI,
                        "LI" => Opcode::PUSH_IMMEDIATE_LI,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for PUSH immediate 8 bit: {}",
                                token_2.raw,
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
                        "AB" => Opcode::PUSH_IMMEDIATE_AB,
                        "HLI" => Opcode::PUSH_IMMEDIATE_HLI,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for PUSH immediate 16 bit: {}",
                                token_1.raw,
                            ))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (
                    TokenType::Instruction,
                    TokenType::Address | TokenType::Text | TokenType::Label,
                    TokenType::Register,
                ) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::PUSH_ABSOLUTE_A,
                        "B" => Opcode::PUSH_ABSOLUTE_B,
                        "HI" => Opcode::PUSH_ABSOLUTE_HI,
                        "LI" => Opcode::PUSH_ABSOLUTE_LI,
                        "AB" => Opcode::PUSH_ABSOLUTE_AB,
                        "HLI" => Opcode::PUSH_ABSOLUTE_HLI,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for PUSH absolute: {}",
                                token_2.raw,
                            ))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::Indirection, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::PUSH_INDIRECT_A,
                        "B" => Opcode::PUSH_INDIRECT_B,
                        "AB" => Opcode::PUSH_INDIRECT_AB,
                        _ => return Err(anyhow!("indirect PUSH can only be used with A, B or AB")),
                    });
                    Ok(vec![instruction])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Register) => {
                    instruction.opcode = Some(
                        match (
                            token_1.formatted_raw().as_str(),
                            token_2.formatted_raw().as_str(),
                        ) {
                            ("A", "B") => Opcode::PUSH_A_B,
                            ("A", "S") => Opcode::PUSH_A_STACK,
                            ("B", "A") => Opcode::PUSH_B_A,
                            ("B", "S") => Opcode::PUSH_B_STACK,
                            ("HI", "A") => Opcode::PUSH_HI_A,
                            ("HI", "B") => Opcode::PUSH_HI_B,
                            ("HI", "S") => Opcode::PUSH_HI_STACK,
                            ("LI", "A") => Opcode::PUSH_LI_A,
                            ("LI", "B") => Opcode::PUSH_LI_B,
                            ("LI", "S") => Opcode::PUSH_LI_STACK,
                            ("EX", "A") => Opcode::PUSH_EXIT_CODE_A,
                            ("EX", "B") => Opcode::PUSH_EXIT_CODE_A,
                            ("SA", "AB") => Opcode::PUSH_AB_STACK_ADDRESS,
                            ("SS", "AB") => Opcode::PUSH_AB_STACK_SIZE,
                            ("HLI", "AB") => Opcode::PUSH_HLI_AB,
                            _ => {
                                return Err(anyhow!(
                                    "incorrect register sequence for PUSH: {} {}",
                                    token_1.raw,
                                    token_2.raw
                                ))
                            }
                        },
                    );
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => todo!(),
    }
}
