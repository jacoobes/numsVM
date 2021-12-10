# numsVM

numsVM is a simple stack based virtual machine implemented in [Rust](https://www.rust-lang.org)
- I did copy a lot from [Crafting Interpreter's chapter on virtual machines](http://craftinginterpreters.com/a-virtual-machine.html)
- supports primtive types
  - f64, f32, i64, i32, strings, chars, booleans
      - in the form of tagged unions
- statically typed primitive binary operations

# Not yet implemented
- [ ]  optimizing primitive data types
- [ ]  macros for reducing repeated code
- [ ]  better disassembler
- [ ]  better error handling
- [ ]  optimizing instructions / more instructions
- [ ]  better error handling, line reporting
- [ ]  stack frames (when functions come)
- [ ]  much more!
