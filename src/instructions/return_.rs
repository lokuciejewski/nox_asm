use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token, TokenType};

pub(crate) fn parse_return(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if let Some(target_flag_token) = tokenised_line.get(1) {
        if target_flag_token._type == TokenType::Flag {
            match target_flag_token.raw.to_uppercase().as_str() {
                "OK" => {
                    if let Some(exit_code_token) = tokenised_line.get(2) {
                        // RET OK #<8BIT>
                        instruction.opcode = Some(Opcode::RETURN_OK_EXIT_CODE);
                        let mut target = exit_code_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        Ok(vec![instruction, target])
                    } else {
                        // RET OK
                        instruction.opcode = Some(Opcode::RETURN_OK);
                        Ok(vec![instruction])
                    }
                }
                "ERR" => {
                    if let Some(exit_code_token) = tokenised_line.get(2) {
                        // RET ERR #<8BIT>
                        instruction.opcode = Some(Opcode::RETURN_ERR_EXIT_CODE);
                        let mut target = exit_code_token.clone();
                        *current_mem_address += 1;
                        target.address = Some(*current_mem_address);
                        Ok(vec![instruction, target])
                    } else {
                        // RET ERR
                        instruction.opcode = Some(Opcode::RETURN_ERR);
                        Ok(vec![instruction])
                    }
                }
                _ => Err(anyhow!(
                    "{} is not a valid flag for RET",
                    target_flag_token.raw
                )),
            }
        } else {
            Err(anyhow!("{} is not a flag argument", target_flag_token.raw))
        }
    } else {
        Err(anyhow!("RET requires at least one flag argument (OK/ERR)"))
    }
}
