
#[cfg(test)]
mod test {

    use crate::bytecode::chunk::Chunk;
    use crate::bytecode::data::Data;
    use crate::bytecode::data::Type;
    use crate::bytecode::disassemble::disassemble_chunk;
    use crate::bytecode::op_code::OpCode::*;
    use crate::vm::nums_vm::NumsVM;
//basic, default chunk    
fn load_const_chunk(data: Data) -> Chunk {
    let mut chunk = Chunk::new();
    let location_of_const = chunk.add_const(data);
    chunk.write(&LOAD_CONST);
    chunk.write_const(location_of_const);
    chunk
}
fn create_vm<'a> (chunk: Chunk) -> () {
    match NumsVM::new(chunk).eval(true) {
        Ok(_) => (),
        Err(e) => panic!("{}", &e),
    }
    
}    
#[test]
fn iter () {

   let chunk = load_const_chunk(Data::new(Type::F64(1.3)));
    create_vm(chunk);
}
#[test]
fn add () {
    let mut chunk = load_const_chunk(Data::new(Type::F64(1.5)));
    let loc_of_const2 = chunk.add_const(Data::new(Type::F64(231.223)));
    chunk.write_const(loc_of_const2);
    chunk.write(&LOAD_CONST);
    chunk.write(&ADD);
    disassemble_chunk(&chunk);    
}
#[test]
#[should_panic(expected = "mismatch type adding")]
fn add_panic_mismatch_type() {
    let mut chunk = load_const_chunk(Data::new(Type::F64(1.5)));
    let loc_of_const2 = chunk.add_const(Data::new(Type::F32(231.223)));
    chunk.write_const(loc_of_const2);
    chunk.write(&LOAD_CONST);
    chunk.write(&ADD);
    create_vm(chunk);
    
}
#[test]
fn sub () {
    let mut chunk = load_const_chunk(Data::new(Type::F64(1.5)));
    let loc_of_const2 = chunk.add_const(Data::new(Type::F64(231.223)));
    chunk.write_const(loc_of_const2);
    chunk.write(&LOAD_CONST);
    chunk.write(&SUB);
    disassemble_chunk(&chunk);    
    create_vm(chunk);
}
#[test]
fn mult () {
    let mut chunk = Chunk::new();
    let location_of_const = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(location_of_const);
        chunk.write(&NEGATE);
    let loc_const2 = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(loc_const2);
        chunk.write(&ADD);
    disassemble_chunk(&chunk);    
    create_vm(chunk);
}
#[test]
fn div() {
    let mut chunk = Chunk::new();
    let location_of_const = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(location_of_const);
        chunk.write(&NEGATE);
    let loc_const2 = chunk.add_const(Data::new(Type::F64(1.2)));
        chunk.write(&LOAD_CONST);
        chunk.write_const(loc_const2);
        chunk.write(&ADD);
    disassemble_chunk(&chunk);    
    create_vm(chunk);
}


}
