use super::{
    chunk::Chunk,
    op_code::OpCode,
};
use crate::bytecode::op_code::OpCode::*;

pub fn disassemble_chunk(chunk: &Chunk) {
    println!("{:=^40}", " test chunk ");

    let mut offset: usize = 0;

    while offset < chunk.instructions.len() {
        offset = disassemble_instruction(chunk, &mut offset)
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: &mut usize) -> usize {
    let instruction = chunk.instructions.get(*offset).expect("expected an opcode");

    match instruction {
        0x00 => simple_instruction(offset, HALT),
        0x01 => const_instruction(chunk, offset, LOAD_CONST),
        0x02 => simple_instruction(offset, NEGATE),
        0x03 => simple_instruction(offset, ADD),
        0x04 => simple_instruction(offset, SUB),
        0x05 => simple_instruction(offset, MUL),
        0x06 => simple_instruction(offset, DIV),
        _ => {
            panic!("unknown opcode!")

        }
    }
}

fn simple_instruction(offset: &mut usize, instruction: OpCode) -> usize {
    println!(
        "{:#04x}{:>20}",
        Chunk::op_to_u8(&instruction),
        format!("{:?}", instruction)
    );
    *offset + 1
}
fn const_instruction(chunk: &Chunk, offset: &mut usize, instruction: OpCode) -> usize {
    let loc_of_const = chunk.instructions[*offset + 1] as usize;
    print!(
        "{:#04x}{:>20}",
        Chunk::op_to_u8(&instruction),
        format!("{:?}", instruction)
    );
    print!("{:>10}", &loc_of_const);
    println!("{:?}",chunk.constants[loc_of_const]);
    *offset + 2
}
