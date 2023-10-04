use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_cmp(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // CMP <REG> #<8HEX> | CMP <REG> #<16HEX> | CMP <REG> <REG> | CMP <REG> <16HEX>
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(source_reg_token) = tokenised_line.get(1) {
        if let Some(target_token) = tokenised_line.get(2) {
            match (&source_reg_token._type, &target_token._type) {
                (TokenType::Register, TokenType::Register) => {
                    if source_reg_token.raw.to_uppercase() == "A"
                        && target_token.raw.to_uppercase() == "B"
                    {
                        instruction.opcode = Some(Opcode::CMP_A_B);
                        Ok(vec![instruction])
                    } else {
                        Err(anyhow!("CMP may only be used as `CMP A B`"))
                    }
                }
                (TokenType::Register, TokenType::ImmediateValue8) => {
                    match source_reg_token.raw.to_uppercase().as_str() {
                        "A" => {
                            instruction.opcode = Some(Opcode::CMP_A_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            Ok(vec![instruction, target_val])
                        }
                        "B" => {
                            instruction.opcode = Some(Opcode::CMP_B_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            Ok(vec![instruction, target_val])
                        }
                        "HI" => {
                            instruction.opcode = Some(Opcode::CMP_HI_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            Ok(vec![instruction, target_val])
                        }
                        "LI" => {
                            instruction.opcode = Some(Opcode::CMP_LI_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            Ok(vec![instruction, target_val])
                        }
                        _ => Err(anyhow!("syntax error")),
                    }
                }
                (TokenType::Register, TokenType::ImmediateValue16) => {
                    match source_reg_token.raw.to_uppercase().as_str() {
                        "AB" => {
                            instruction.opcode = Some(Opcode::CMP_AB_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            *current_mem_address += 1;
                            Ok(vec![instruction, target_val])
                        }
                        "HLI" => {
                            instruction.opcode = Some(Opcode::CMP_HLI_IMMEDIATE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            *current_mem_address += 1;
                            Ok(vec![instruction, target_val])
                        }
                        _ => Err(anyhow!("syntax error")),
                    }
                }
                (TokenType::Register, TokenType::Address) => {
                    match source_reg_token.raw.to_uppercase().as_str() {
                        "AB" => {
                            instruction.opcode = Some(Opcode::CMP_AB_ABSOLUTE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            *current_mem_address += 1;
                            Ok(vec![instruction, target_val])
                        }
                        "HLI" => {
                            instruction.opcode = Some(Opcode::CMP_HLI_ABSOLUTE);
                            let mut target_val = target_token.clone();
                            *current_mem_address += 1;
                            target_val.address = Some(*current_mem_address);
                            *current_mem_address += 1;
                            Ok(vec![instruction, target_val])
                        }
                        _ => Err(anyhow!("syntax error")),
                    }
                }
                (_, _) => Err(anyhow!(
                    "invalid token in CMP instruction: {}/{}",
                    source_reg_token.raw,
                    target_token.raw
                )),
            }
        } else {
            Err(anyhow!("syntax error"))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
