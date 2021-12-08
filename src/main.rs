pub mod bytecode;
pub mod vm;

use bytecode::disassemble::{self, disassemble_chunk};

use crate::bytecode::data::{Data, Type};

use crate::bytecode::op_code::OpCode::{*};
fn main() {
    let mut chunk = bytecode::chunk::Chunk::new();

    let location_of_const = chunk.add_const(Data::new(Type::F64(1.2))) as u8;
    chunk.write(&LOAD_CONST);
    chunk.write_const(location_of_const);
    chunk.write(&HALT); 
    println!("{:?}", &chunk);   
    disassemble_chunk(&chunk)
}
