use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_sto(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // STO <REG> <16HEX>
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(source_reg_token) = tokenised_line.get(1) {
        if source_reg_token._type == TokenType::Register {
            // Second token is a register
            if let Some(target_token) = tokenised_line.get(2) {
                if target_token._type == TokenType::Address {
                    instruction.opcode = Some(match source_reg_token.raw.to_uppercase().as_str() {
                        "HI" => Opcode::STORE_HI_ABSOLUTE,
                        "LI" => Opcode::STORE_LI_ABSOLUTE,
                        "HLI" => Opcode::STORE_HLI_ABSOLUTE,
                        _ => {
                            return Err(anyhow!(
                                "STO may only be used with HI, LI or HLI registers"
                            ))
                        }
                    });
                    let mut target = target_token.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                } else {
                    Err(anyhow!("token {} is not a valid address", target_token.raw))
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
