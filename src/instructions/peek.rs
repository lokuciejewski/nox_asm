use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_peek(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut tokens = tokenised_line
        .get(0..=2)
        .ok_or_else(|| anyhow!("too few tokens in line for PUSH"))?
        .to_owned();

    match tokens.as_mut_slice() {
        [instruction, token_1, token_2] => {
            let mut instruction = instruction.clone();
            instruction.address = Some(*current_mem_address);
            match (&instruction._type, &token_1._type, &token_2._type) {
                (
                    TokenType::Instruction,
                    TokenType::Register,
                    TokenType::Address | TokenType::Text | TokenType::Label,
                ) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::PEEK_A_ABSOLUTE,
                        "B" => Opcode::PEEK_B_ABSOLUTE,
                        "AB" => Opcode::PEEK_AB_ABSOLUTE,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for PEEK absolute: {}",
                                token_1.raw,
                            ))
                        }
                    });
                    let mut target = token_2.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Indirection) => {
                    instruction.opcode = Some(match token_1.formatted_raw().as_str() {
                        "A" => Opcode::PEEK_A_INDIRECT,
                        "B" => Opcode::PEEK_B_INDIRECT,
                        "AB" => Opcode::PEEK_AB_INDIRECT,
                        _ => return Err(anyhow!("indirect POP can only be used with A, B or AB")),
                    });
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => todo!(),
    }
}
