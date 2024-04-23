use crate::{
    chunk::{ByteCode, Chunk},
    values::Value,
};

pub type VMResult = Result<(), VMError>;

#[derive(Debug)]
pub enum VMError {
    CompError,
    RuntimeError,
}

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VirtualMachine<'a> {
    pub fn run(chunk: &'a Chunk) -> VMResult {
        let mut vm: VirtualMachine<'a> = Self {
            chunk,
            ip: 0,
            stack: vec![],
        };
        while let Ok(instruction) = vm.read_byte() {
            dbg!(TryInto::<ByteCode>::try_into(instruction));
            match TryInto::<ByteCode>::try_into(instruction) {
                Ok(byte_code) => {
                    match byte_code {
                        ByteCode::Return => VMResult::Ok(())?,
                        ByteCode::Constant => {
                            let constant = vm.read_constant()?;
                            vm.stack.push(constant);
                        }
                        ByteCode::Negate => {
                            let source = vm.read_u16()?;
                            let dest = vm.read_u16()?;
                            *(vm.stack.get_mut(dest as usize).ok_or(VMError::RuntimeError)?) = -vm.stack.get(source as usize).ok_or(VMError::RuntimeError)?.clone();
                        }
                    };
                }
                Err(_) => return VMResult::Err(VMError::RuntimeError),
            }
        }
        dbg!(vm.stack);
        VMResult::Ok(())
    }
    fn read_byte(&mut self) -> Result<u8, VMError> {
        self.chunk
            .code
            .get({
                let a = self.ip;
                self.ip += 1;
                a
            })
            .copied()
            .ok_or(VMError::RuntimeError)
    }
    fn read_constant(&mut self) -> Result<Value, VMError> {
        let constant_index = self.read_byte()? as usize;
        self.chunk
            .constants
            .get(constant_index)
            .cloned()
            .ok_or(VMError::RuntimeError)
    }
    fn read_u16(&mut self) -> Result<u16, VMError> {
        let top_bytes = self.read_byte()?;
        let bottom_bytes = self.read_byte()?;
        Ok((top_bytes as u16 >> 8) | (bottom_bytes as u16))
    }
}
