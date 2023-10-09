use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

#[derive(PartialEq)]
pub(crate) enum Direction {
    Left,
    Right,
}

pub(crate) fn parse_shift(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
    shift_direction: Direction,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_reg_token) = tokenised_line.get(1) {
        if target_reg_token._type == TokenType::Register {
            // Second token must be a register
            match target_reg_token.formatted_raw().as_str() {
                "A" => {
                    instruction.opcode = Some(if shift_direction == Direction::Left {
                        Opcode::SHIFT_LEFT_A
                    } else {
                        Opcode::SHIFT_RIGHT_A
                    });
                }
                "B" => {
                    instruction.opcode = Some(if shift_direction == Direction::Left {
                        Opcode::SHIFT_LEFT_B
                    } else {
                        Opcode::SHIFT_RIGHT_B
                    });
                }
                "AB" => {
                    instruction.opcode = Some(if shift_direction == Direction::Left {
                        Opcode::SHIFT_LEFT_AB
                    } else {
                        Opcode::SHIFT_RIGHT_AB
                    });
                }
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
