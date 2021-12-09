use bytecode::{chunk::Chunk, data::{Type, Data}};
use vm::vm::VM;
use crate::bytecode::op_code::OpCode::{*};
pub mod bytecode;
pub mod vm;
mod tests;



fn main() {
    let mut chunk = Chunk::new();
        let location_of_const = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(location_of_const);
        chunk.write(&HALT);
    println!("{:?}", chunk);
    VM::new(chunk).exec()

}
