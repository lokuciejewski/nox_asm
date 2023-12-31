use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_call(
    tokenised_line: &[Token],
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_token) = tokenised_line.get(1) {
        if target_token._type == TokenType::Address
            || target_token._type == TokenType::Label
            || target_token._type == TokenType::Text
        {
            instruction.opcode = Some(Opcode::CALL);
            let mut target_token_clone = target_token.clone();
            *current_mem_address += 1;
            target_token_clone.address = Some(*current_mem_address);
            *current_mem_address += 1;
            Ok(vec![instruction, target_token_clone])
        } else {
            Err(anyhow!(
                "token {} is not an address or a label",
                target_token.raw
            ))
        }
    } else {
        Err(anyhow!("syntax error"))
    }
}
