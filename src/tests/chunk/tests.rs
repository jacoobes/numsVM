#[cfg(test)]
mod tests {
    use crate::bytecode::data::{Data, Type};
    use crate::bytecode::disassemble::disassemble_chunk;
    use crate::bytecode::op_code::OpCode::*;
    use crate::bytecode::chunk::*;

    #[test]
    fn write_chunk() {
       let mut chunk = Chunk::new();
        let location_of_const = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(location_of_const);
        chunk.write(&HALT);
        
        disassemble_chunk(&chunk)
    }



}