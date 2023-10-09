use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_swap(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    // ADD <REG> <REG> | ADD <REG> #<8BIT> | ADD <REG> <16BIT>
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
                                target_token.formatted_raw().as_str(),
                                source_reg_token.formatted_raw().as_str(),
                            ) {
                                ("HI", "LI") | ("LI", "HI") => Opcode::SWAP_HI_LI,
                                _ => return Err(anyhow!("SWP only works for HI and LI")),
                            },
                        );
                        Ok(vec![instruction])
                    }
                    _ => Err(anyhow!("SWP only works for HI and LI")),
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
