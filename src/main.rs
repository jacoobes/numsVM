pub mod bytecode;
pub mod vm;
mod tests;

use bytecode::disassemble::{self, disassemble_chunk};

use crate::bytecode::data::{Data, Type};

use crate::bytecode::op_code::OpCode::{*};
fn main() {
    let mut chunk = bytecode::chunk::Chunk::new();

}
