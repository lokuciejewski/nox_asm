use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_not(
    tokenised_line: &[Token],
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_reg_token) = tokenised_line.get(1) {
        if target_reg_token._type == TokenType::Register {
            // Second token must be a register
            match target_reg_token.formatted_raw().as_str() {
                "A" => instruction.opcode = Some(Opcode::NOT_A),
                "B" => instruction.opcode = Some(Opcode::NOT_B),
                "AB" => instruction.opcode = Some(Opcode::NOT_AB),
                _ => return Err(anyhow!("syntax error")),
            }
            Ok(vec![instruction])
        } else {
            Err(anyhow!("token {} is not a register", target_reg_token.raw))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
