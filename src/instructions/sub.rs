use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub (crate) fn parse_sub(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // SUB <REG> <REG> | SUB <REG> #<8BIT> | SUB <REG> <16BIT> 
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(source_reg_token) = tokenised_line.get(1) {
        if source_reg_token._type == TokenType::Register {
            // Second token is a register
            if let Some(target_token) = tokenised_line.get(2) {
                match target_token._type {
                    TokenType::Register => {
                        instruction.opcode = Some(
                            match (
                                target_token.raw.to_uppercase().as_str(),
                                source_reg_token.raw.to_uppercase().as_str(),
                            ) {
                                ("A", "B") => Opcode::SUB_A_B,
                                ("B", "A") => Opcode::SUB_B_A,
                                _ => return Err(anyhow!("only A and B can be subtracted together"))
                            },
                        );
                        Ok(vec![instruction])
                    }
                    TokenType::ImmediateValue8 => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "A" => Opcode::SUB_A_IMMEDIATE,
                                "B" => Opcode::SUB_B_IMMEDIATE,
                                _ => return Err(anyhow!("only A and B can be used with immediate 8 bit value"))
                            });
                        let mut target = target_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        Ok(vec![instruction, target])
                    }
                    TokenType::ImmediateValue16 => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "AB" => Opcode::SUB_AB_IMMEDIATE,
                                _ => return Err(anyhow!("only AB can be used with immediate 16 bit value")),
                            });
                        let mut target = target_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        *current_mem_address += 1; // 2 byte value
                        Ok(vec![instruction, target])
                    }
                    TokenType::Address => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "A" => Opcode::SUB_A_ABSOLUTE,
                                "B" => Opcode::SUB_B_ABSOLUTE,
                                "AB" => Opcode::SUB_AB_ABSOLUTE,
                                _ => return Err(anyhow!("only A, B or AB can be used in SUB absolute")),
                            });
                        let mut target = target_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        *current_mem_address += 1; // 2 byte value
                        Ok(vec![instruction, target])
                    }
                    _ => Err(anyhow!(
                        "token {} is not a register/address/immediate value",
                        target_token.raw
                    )),
                }
            } else {
                Err(anyhow!("syntax error"))
            }
        } else {
            Err(anyhow!("token {} is not a register", source_reg_token.raw))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
