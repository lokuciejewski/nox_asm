use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_zero(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_reg_token) = tokenised_line.get(1) {
        if target_reg_token._type == TokenType::Register {
            // Second token is a register
            match target_reg_token.raw.to_uppercase().as_str() {
                "HI" => {
                    instruction.opcode = Some(Opcode::ZERO_HI);
                    Ok(vec![instruction])
                }
                "LI" => {
                    instruction.opcode = Some(Opcode::ZERO_LI);
                    Ok(vec![instruction])
                }
                "HLI" => {
                    instruction.opcode = Some(Opcode::ZERO_HLI);
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        } else {
            Err(anyhow!("token {} is not a register", target_reg_token.raw))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
