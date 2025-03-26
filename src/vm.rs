use crate::codegen::{Instruction, Object};

pub struct VM {
    constants: Vec<Object>,
    globals: Vec<Object>,
    stack: Vec<Object>,
    sp: usize,
    frams: Vec<Frame>,
    frame_index: usize,
}

struct Frame {
    instructions: Vec<Instruction>,
    ip: usize,
    base_Pointer: usize
}