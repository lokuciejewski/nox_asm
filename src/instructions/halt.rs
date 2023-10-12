use anyhow::{anyhow, Error};

use crate::{opcodes::Opcode, Token};

pub(crate) fn parse_halt(
    tokenised_line: &Vec<Token>,
    current_mem_address: &mut u16,
) -> Result<Vec<Token>, Error> {
    let mut instruction = tokenised_line.get(0).unwrap().clone();
    instruction.address = Some(*current_mem_address);
    if tokenised_line.get(1).is_none() {
        instruction.opcode = Some(Opcode::HALT);
        Ok(vec![instruction])
    } else {
        Err(anyhow!("HALT should be used without any arguments"))
    }
}
