use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_inc(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_reg_token) = tokenised_line.get(1) {
        if target_reg_token._type == TokenType::Register {
            // Second token is a register
            instruction.opcode = Some(match target_reg_token.raw.to_uppercase().as_str() {
                "HI" => Opcode::INC_HI,
                "LI" => Opcode::INC_LI,
                "HLI" => Opcode::INC_HLI,
                _ => Opcode::NOOP,
            });
            Ok(vec![instruction])
        } else {
            Err(anyhow!("token {} is not a register", target_reg_token.raw))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
