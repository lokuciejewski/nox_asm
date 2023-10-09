use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_or(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(source_reg_token) = tokenised_line.get(1) {
        if source_reg_token._type == TokenType::Register {
            // Second token is a register
            match source_reg_token.raw.to_uppercase().as_str() {
                "A" | "B" => {
                    if let Some(target_reg_token) = tokenised_line.get(2) {
                        if target_reg_token._type == TokenType::Register {
                            match (
                                source_reg_token.raw.to_uppercase().as_str(),
                                target_reg_token.raw.to_uppercase().as_str(),
                            ) {
                                ("A", "B") => {
                                    instruction.opcode = Some(Opcode::OR_A_B);
                                    Ok(vec![instruction])
                                }
                                ("B", "A") => {
                                    instruction.opcode = Some(Opcode::OR_B_A);
                                    Ok(vec![instruction])
                                }
                                _ => {
                                    return Err(anyhow!(
                                        "AND can only be used on A and B registers"
                                    ))
                                }
                            }
                        } else {
                            Err(anyhow!("token {} is not a register", target_reg_token.raw))
                        }
                    } else {
                        Err(anyhow!("syntax error"))
                    }
                }
                "AB" => {
                    if let Some(target_reg_token) = tokenised_line.get(2) {
                        instruction.opcode = Some(match target_reg_token._type {
                            TokenType::Address => Opcode::OR_AB_ABSOLUTE,
                            TokenType::ImmediateValue16 => Opcode::OR_AB_IMMEDIATE,
                            _ => {
                                return Err(anyhow!(
                                "AND AB can only be used with address or immediate 16 bit value"
                            ))
                            }
                        });
                        let mut target = target_reg_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        *current_mem_address += 1; // 2 byte value
                        Ok(vec![instruction, target])
                    } else {
                        Err(anyhow!("syntax error"))
                    }
                }
                _ => Err(anyhow!("syntax error")),
            }
        } else {
            Err(anyhow!("token {} is not a register", source_reg_token.raw))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
