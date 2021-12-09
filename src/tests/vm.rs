
#[cfg(test)]
mod test {
    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::data::Data;
    use crate::bytecode::data::Type;
    use crate::bytecode::op_code::OpCode::*;
    use crate::vm::vm::VM;
    
#[test]
fn iter () {

    let mut chunk = Chunk::new();
    let location_of_const = chunk.add_const(Data::new(Type::F64(1.2)));
    chunk.write(&LOAD_CONST);
    chunk.write_const(location_of_const);
    chunk.write(&NEGATE);
    chunk.write(&HALT);

    VM::new(chunk).exec(false);
}

}
