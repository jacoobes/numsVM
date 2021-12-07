pub mod bytecode;
pub mod vm;

use crate::bytecode::chunk::OperatorCode::HALT;
fn main() {
    println!("Hello, world!");
    let mut chunk = bytecode::chunk::Chunk::new();
    chunk.write(&HALT);
    
}
