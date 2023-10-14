use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_pop(
    tokenised_line: &[Token],
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    match (
        tokenised_line.get(0),
        tokenised_line.get(1),
        tokenised_line.get(2),
    ) {
        (Some(instruction), Some(token_1), Some(token_2)) => {
            let mut instruction = instruction.clone();
            instruction.address = Some(*current_mem_address);
            match (&instruction._type, &token_1._type, &token_2._type) {
                (
                    TokenType::Instruction,
                    TokenType::Register,
                    TokenType::Address | TokenType::Text | TokenType::Label,
                ) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::POP_A_ABSOLUTE,
                        "B" => Opcode::POP_B_ABSOLUTE,
                        "AB" => Opcode::POP_AB_ABSOLUTE,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for POP absolute: {}",
                                token_1.raw,
                            ))
                        }
                    });
                    let mut target = token_2.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Indirection) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::POP_A_INDIRECT,
                        "B" => Opcode::POP_B_INDIRECT,
                        "AB" => Opcode::POP_AB_INDIRECT,
                        _ => return Err(anyhow!("indirect POP can only be used with A, B or AB")),
                    });
                    Ok(vec![instruction])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Register) => {
                    instruction.opcode = Some(
                        match (
                            token_1.formatted_raw().as_str(),
                            token_2.formatted_raw().as_str(),
                        ) {
                            ("A", "HI") => Opcode::POP_A_HI,
                            ("A", "LI") => Opcode::POP_A_LI,
                            ("B", "HI") => Opcode::POP_B_HI,
                            ("B", "LI") => Opcode::POP_B_LI,
                            ("A", "B") => Opcode::POP_A_B,
                            ("B", "A") => Opcode::POP_B_A,
                            ("AB", "SA") => Opcode::POP_STACK_ADDRESS_AB,
                            ("AB", "SS") => Opcode::POP_STACK_SIZE_AB,
                            ("AB", "IRA") => Opcode::POP_AB_IRQ,
                            ("AB", "HLI") => Opcode::POP_AB_HLI,
                            ("A", "S") => Opcode::POP_A_STACK,
                            ("B", "S") => Opcode::POP_B_STACK,
                            ("S", "A") => Opcode::POP_STACK_A,
                            ("S", "B") => Opcode::POP_STACK_B,
                            ("S", "HI") => Opcode::POP_STACK_HI,
                            ("S", "LI") => Opcode::POP_STACK_LI,
                            _ => {
                                return Err(anyhow!(
                                    "incorrect register sequence for POP: {} {}",
                                    token_1.raw,
                                    token_2.raw
                                ))
                            }
                        },
                    );
                    match instruction.opcode.as_ref().unwrap() {
                        Opcode::POP_STACK_A
                        | Opcode::POP_STACK_B
                        | Opcode::POP_A_STACK
                        | Opcode::POP_B_STACK => {
                            if let Some(token_3) = tokenised_line.get(3) {
                                if token_3._type == TokenType::ImmediateValue8 {
                                    let mut target = token_3.clone();
                                    *current_mem_address += 1;
                                    target.address = Some(*current_mem_address);
                                    Ok(vec![instruction, target])
                                } else {
                                    Err(anyhow!(
                                        "POP {} {} needs an immediate value",
                                        token_1.raw,
                                        token_2.raw
                                    ))
                                }
                            } else {
                                Err(anyhow!(
                                    "POP {} {} needs an immediate value",
                                    token_1.raw,
                                    token_2.raw
                                ))
                            }
                        }
                        _ => Ok(vec![instruction]),
                    }
                }
                (TokenType::Instruction, TokenType::Register, TokenType::CommentStart) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::POP_A,
                        "B" => Opcode::POP_B,
                        _ => return Err(anyhow!("one argument POP can only be used with A or B")),
                    });
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        (Some(instruction), Some(token_1), None) => {
            let mut instruction = instruction.clone();
            instruction.address = Some(*current_mem_address);
            match (&instruction._type, &token_1._type) {
                (TokenType::Instruction, TokenType::Register) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::POP_A,
                        "B" => Opcode::POP_B,
                        _ => return Err(anyhow!("one argument POP can only be used with A or B")),
                    });
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => Err(anyhow!("line parsing error")),
    }
}
