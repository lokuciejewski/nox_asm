use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_set(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_flag_token) = tokenised_line.get(1) {
        if target_flag_token._type == TokenType::Flag {
            instruction.opcode = Some(match target_flag_token.formatted_raw().as_str() {
                "ERR" => Opcode::SET_ERR,
                "IRQ" => Opcode::SET_IRQ,
                _ => {
                    return Err(anyhow!(
                        "{} is not a valid flag for SET",
                        target_flag_token.raw
                    ))
                }
            });
            Ok(vec![instruction])
        } else {
            Err(anyhow!("{} is not a flag argument", target_flag_token.raw))
        }
    } else {
        Err(anyhow!("SET requires at least one flag argument"))
    }
}
