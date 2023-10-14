use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_cmp(
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
                (TokenType::Instruction, TokenType::Register, TokenType::Register) => {
                    instruction.opcode = Some(
                        match (
                            token_1.formatted_raw().as_str(),
                            token_2.formatted_raw().as_str(),
                        ) {
                            ("A", "B") => Opcode::CMP_A_B,
                            _ => return Err(anyhow!("CMP may only be used as `CMP A B`")),
                        },
                    );
                    Ok(vec![instruction])
                }
                (TokenType::Instruction, TokenType::ImmediateValue8, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::CMP_IMMEDIATE_A,
                        "B" => Opcode::CMP_IMMEDIATE_B,
                        "HI" => Opcode::CMP_IMMEDIATE_HI,
                        "LI" => Opcode::CMP_IMMEDIATE_LI,
                        reg => return Err(anyhow!("invalid register for 8 bit CMP: {}", reg)),
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    Ok(vec![instruction, target])
                }
                (TokenType::Instruction, TokenType::ImmediateValue16, TokenType::Register) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "AB" => Opcode::CMP_IMMEDIATE_AB,
                        "HLI" => Opcode::CMP_IMMEDIATE_HLI,
                        reg => return Err(anyhow!("invalid register for 16 bit CMP: {}", reg)),
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1;
                    Ok(vec![instruction, target])
                }
                (
                    TokenType::Instruction,
                    TokenType::Address | TokenType::Text | TokenType::Label,
                    TokenType::Register,
                ) => {
                    instruction.opcode = Some(match token_2.formatted_raw().as_str() {
                        "A" => Opcode::CMP_ABSOLUTE_A,
                        "B" => Opcode::CMP_ABSOLUTE_B,
                        "HI" => Opcode::CMP_ABSOLUTE_HI,
                        "LI" => Opcode::CMP_ABSOLUTE_LI,
                        "AB" => Opcode::CMP_ABSOLUTE_AB,
                        "HLI" => Opcode::CMP_ABSOLUTE_HLI,
                        reg => return Err(anyhow!("invalid register for CMP absolute: {}", reg)),
                    });
                    let mut target = token_1.clone();
                    *current_mem_address += 1;
                    target.address = Some(*current_mem_address);
                    *current_mem_address += 1;
                    Ok(vec![instruction, target])
                }
                _ => Err(anyhow!("syntax error")),
            }
        }
        _ => todo!(),
    }
}
