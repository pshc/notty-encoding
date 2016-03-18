use args::{Argument, InputSettings};
use cmds::EscCode;

pub struct SetTitle(pub String);

impl EscCode for SetTitle {
    const OPCODE: u16 = 0x40;
    fn attachments(&self) -> Vec<Vec<u8>> {
        vec![self.0.clone().into_bytes()]
    }
}

pub struct PushBuffer(pub bool);

impl EscCode for PushBuffer {
    const OPCODE: u16 = 0x60;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct PopBuffer;

impl EscCode for PopBuffer {
    const OPCODE: u16 = 0x61;
}

pub struct SetInputMode(pub InputSettings);

impl EscCode for SetInputMode {
    const OPCODE: u16 = 0x80;
    fn args(&self) -> Vec<String> {
        vec![self.0.encode()]
    }
}

pub struct HoldForInput;

impl EscCode for HoldForInput {
    const OPCODE: u16 = 0x87;
}
