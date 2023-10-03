use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub (crate) fn parse_push(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // PUSH <REG> #<8HEX> | PUSH <REG> #<16HEX> | PUSH <REG> <REG> | PUSH <REG> <16HEX>
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
                                ("AB", "SA") => Opcode::PUSH_AB_STACK_ADDRESS,
                                ("AB", "SS") => Opcode::PUSH_AB_STACK_SIZE,
                                ("HLI", "AB") => Opcode::PUSH_HLI_AB,
                                _ => Opcode::NOOP,
                            },
                        );
                        Ok(vec![instruction])
                    }
                    TokenType::ImmediateValue8 => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "A" => Opcode::PUSH_A_IMMEDIATE,
                                "B" => Opcode::PUSH_B_IMMEDIATE,
                                "HI" => Opcode::PUSH_HI_IMMEDIATE,
                                "LI" => Opcode::PUSH_LI_IMMEDIATE,
                                _ => Opcode::NOOP,
                            });
                        let mut target = target_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        Ok(vec![instruction, target])
                    }
                    TokenType::ImmediateValue16 => {
                        instruction.opcode =
                            Some(match source_reg_token.raw.to_uppercase().as_str() {
                                "AB" => Opcode::PUSH_AB_IMMEDIATE,
                                "HLI" => Opcode::PUSH_HLI_IMMEDIATE,
                                _ => Opcode::NOOP,
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
                                "A" => Opcode::PUSH_A_ABSOLUTE,
                                "B" => Opcode::PUSH_B_ABSOLUTE,
                                "HI" => Opcode::PUSH_HI_ABSOLUTE,
                                "LI" => Opcode::PUSH_LI_ABSOLUTE,
                                "AB" => Opcode::PUSH_AB_ABSOLUTE,
                                "HLI" => Opcode::PUSH_HLI_ABSOLUTE,
                                _ => Opcode::NOOP,
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
