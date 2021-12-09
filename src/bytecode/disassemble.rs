use super::{chunk::Chunk, op_code::OpCode, data::{Type::*, Data}};
use crate::bytecode::op_code::OpCode::{*};


pub fn disassemble_chunk(chunk : &Chunk, ) {
    println!("{:=^40}", " test chunk " );

    let mut offset: usize = 0;
    
    while offset < chunk.instructions.len() {
        offset = disassemble_instruction(chunk, &mut offset)
    }   
    
}

pub fn disassemble_instruction(chunk: &Chunk, offset: &mut usize) -> usize {
    let instruction = chunk.instructions.get(*offset).expect("expected an opcode");
    

    match instruction {
       0x00 => {
        simple_instruction( offset, HALT)
       } 
       0x01 => {
        const_instruction(chunk, offset, LOAD_CONST)
       }
       _ => {
           panic!("unknown opcode!")
       }
    }
}

fn simple_instruction ( offset: &mut usize, instruction: OpCode) -> usize {
    println!("{:#04x}{:>20}", Chunk::op_to_u8(&instruction),   format!("{:?}", instruction));    
    *offset + 1
}
fn const_instruction(chunk : &Chunk, offset: &mut usize, instruction: OpCode) -> usize {
    let loc_of_const = chunk.instructions[*offset + 1] as usize;
    print!("{:#04x}{:>20}", Chunk::op_to_u8(&instruction), format!("{:?}", instruction));
    print!("{:>10}", &loc_of_const);
    print_const(&chunk.constants[loc_of_const]); 
    println!();
    *offset + 2
}

fn print_const(data: &Data) {
    match &data.val {
        F64(v) =>     print!("({:>3})", v),
        F32(v) =>     print!("({:>3})", v),
        I32(v) =>     print!("({:>3})", v),
        I64(v) =>     print!("({:>3})", v),
        Bool(v) =>   print!("({:>3})", v),
        String(v) => print!("({:>3})", v),
        Char(v) =>   print!("({:>3})", v),
    }
}