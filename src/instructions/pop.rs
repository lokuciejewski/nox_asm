use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_pop(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // POP <REG> <REG> | POP <REG> <16HEX>
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
                                ("A", "HI") => Opcode::POP_A_HI,
                                ("A", "LI") => Opcode::POP_A_LI,
                                ("A", "S") => Opcode::POP_A_STACK,
                                ("B", "HI") => Opcode::POP_B_HI,
                                ("B", "LI") => Opcode::POP_B_LI,
                                ("B", "S") => Opcode::POP_B_STACK,
                                ("HI", "S") => Opcode::POP_HI_STACK,
                                ("LI", "S") => Opcode::POP_LI_STACK,
                                ("AB", "SA") => Opcode::POP_AB_STACK_ADDRESS,
                                ("AB", "SS") => Opcode::POP_AB_STACK_SIZE,
                                ("AB", "IRA") => Opcode::POP_AB_IRQ,
                                ("AB", "HLI") => Opcode::POP_AB_HLI,
                                _ => {
                                    return Err(anyhow!(
                                        "incorrect register sequence for POP: {} {}",
                                        source_reg_token.raw,
                                        target_token.raw
                                    ))
                                }
                            },
                        );
                        Ok(vec![instruction])
                    }
                    TokenType::Address => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "A" => Opcode::POP_A_ABSOLUTE,
                                "B" => Opcode::POP_B_ABSOLUTE,
                                "AB" => Opcode::POP_AB_ABSOLUTE,
                                _ => {
                                    return Err(anyhow!(
                                        "incorrect register for POP absolute: {}",
                                        source_reg_token.raw,
                                    ))
                                }
                            });
                        let mut target = target_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        *current_mem_address += 1; // 2 byte value
                        Ok(vec![instruction, target])
                    }
                    _ => Err(anyhow!(
                        "token {} is not a register/address",
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
