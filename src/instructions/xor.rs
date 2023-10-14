use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_xor(
    tokenised_line: &[Token],
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
                    TokenType::Address | TokenType::Text | TokenType::Label,
                    TokenType::Register,
                ) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "AB" => Opcode::XOR_ABSOLUTE_AB,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for AND absolute: {}",
                                token_2.raw,
                            ))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::ImmediateValue16, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "AB" => Opcode::XOR_IMMEDIATE_AB,
                        _ => {
                            return Err(anyhow!(
                                "incorrect register for AND absolute: {}",
                                token_2.raw,
                            ))
                        }
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1; // 2 byte value
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::Register, TokenType::Register) => {
                    instruction.opcode = Some(
                        match (
                            token_1.formatted_raw().as_str(),
                            token_2.formatted_raw().as_str(),
                        ) {
                            ("A", "B") => Opcode::XOR_A_B,
                            ("B", "A") => Opcode::XOR_B_A,
                            _ => {
                                return Err(anyhow!(
                                    "incorrect register sequence for AND: {} {}",
                                    token_1.raw,
                                    token_2.raw
                                ))
                            }
                        },
                    );
                    Ok(vec![instruction])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => todo!(),
    }
}
