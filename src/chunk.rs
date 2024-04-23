use std::mem;

use strum::EnumCount;
use strum_macros::EnumCount;

use crate::values::Value;

#[derive(Debug,EnumCount,Clone, Copy)]
pub enum ByteCode{
    Return,
    Constant,
    Negate,
}
impl Into<u8> for ByteCode{
    fn into(self) -> u8 {
        unsafe{mem::transmute(self)}
    }
}
impl TryInto<ByteCode> for u8{
    type Error=();
    fn try_into(self) -> Result<ByteCode, Self::Error> {
        if self<ByteCode::COUNT as u8{
            Ok(unsafe{
                mem::transmute(self)
            })
        }else{
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct Chunk{
    pub code:Vec<u8>,
    pub constants:Vec<Value>,
}
impl Chunk{
    pub fn new()->Self{
        Self{
            code:vec![],
            constants:vec![],
        }
    }
    pub fn disassemble(&self, name:&str){
        println!("== {name} ==");
        println!("-- constants --");
        self.constants.iter().enumerate().for_each(|(i,value)|{
            println!("{:0>3} {value}",i);
        });
        println!("-- code --");
        let mut offset = 0;
        while offset<self.code.len(){
            offset = (&self).dissasemble_instruction(offset)
        }
    }
    pub fn write_chunk<T:Into<u8>>(&mut self, op_code:T){
        self.code.push(op_code.into())
    }
    pub fn write_constant(&mut self, value:Value)->u8{
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
    fn dissasemble_instruction(&self,mut offset:usize)->usize{
        print!("{:0>4} ",offset);
        let print_args = |airity:usize, offset: usize|->usize{
            println!(" {}",self.code[offset..offset+airity].iter().fold("".into(), |mut string:String,byte|{string.push_str(format!("{:0>3} ",byte).as_str());string}));
            offset+airity
        };
        match TryInto::<ByteCode>::try_into(self.code[offset]){
            Ok(byte_code) => {
                print!("{:0<16?}",byte_code);
                offset+=1;
                match byte_code {
                    ByteCode::Return => print_args(0,offset),
                    ByteCode::Constant => print_args(1,offset),
                    ByteCode::Negate => print_args(4,offset),
                }
            },
            Err(_) => {println!("Unknown Opcode"); offset+1},
        }
    }
}

