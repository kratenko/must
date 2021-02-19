/*
Nat
  f0
  f1
  ff...
Res
  ->Nat
  ->Var
 */

/*
p0: none
p1: OpA,
p2: OpA, OpB
p3: OpA, OpB, OpC
p4: OpA, Im
--
0x20 OpA Im0: OpA <- Im0
 */

use crate::{Res, Value};
use std::rc::Rc;
use core::fmt;
use failure::_core::fmt::Formatter;
use arr_macro::arr;

#[derive(Debug, Clone)]
struct RuntimeError {
    message: String,
}

impl RuntimeError {
    pub fn new(message: &str) -> RuntimeError {
        RuntimeError{
            message: String::from(message),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

type Result<T> = std::result::Result<T, RuntimeError>;

// P0 - None
const OP_TERM: u8 = 0x00;
// P1 - OpA
const OP_LOGV: u8 = 0x10;
// P2 - OpA, OpB
// P3 - OpA, OpB, OpC
const OP_ADD: u8 = 0x80;
const OP_PUSH: u8 = 0x11;
// P4 - OpA, Im
const OP_LOADI: u8 = 0xa0;
const OP_ADDI: u8 = 0xa1;

pub struct Inst {
    pub op: u8,
    pub op_a: u8,
    pub op_b: u8,
    pub op_c: u8,
    pub im: i64,
}

pub struct Frame {
    local_vars: u32,
    fp: usize,
}

impl Frame {
    pub fn new() -> Frame {
        Frame{
            local_vars: 0,
            fp: 0
        }
    }
}

const VSTACK_SIZE: usize = 1024;
pub struct Vm {
    pc: usize,
    sp: usize,
    fsp: usize,
    running: bool,
    vstack: [Value; VSTACK_SIZE],
    fstack: [Frame; 128],
}

impl Vm {
    fn new() -> Vm {
        Vm{
            pc: 0,
            sp: 0,
            fsp: 0,
            running: false,
            vstack: arr![Value::Void; 1024],
            fstack: arr![Frame::new(); 128],
        }
    }

    fn push(&mut self, value: Value) -> Result<()>{
        if self.sp >= VSTACK_SIZE {
            Err(RuntimeError::new("Value stack overflow"))
        } else {
            self.vstack[self.sp] = value;
            self.sp += 1;
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<Value> {
        if self.sp == 0 {
            Err(RuntimeError::new("Value stack underflow"))
        } else {
            self.sp -= 1;
            let value = self.vstack[self.sp].clone();
            self.vstack[self.sp] = Value::Void;
            Ok(value)
        }
    }

    fn load_u8(&mut self, pgm: &[u8]) -> Result<u8> {
        if let Some(op) = pgm.get(self.pc) {
            self.pc += 1;
            Ok(*op)
        } else {
            println!("ERR");
            Err(RuntimeError::new("Unexpected end of program"))
        }
    }

    fn load_vi64(&mut self, pgm: &[u8]) -> Result<i64> {
        let mut v: i64 = 0;
        let mut b = self.load_u8(pgm)?;
        while b & 0b10000000 != 0 {
            v = (v << 7) | (b & 0b01111111) as i64;
            b = self.load_u8(pgm)?;
        }
        Ok((v << 7) | (b & 0b01111111) as i64)
    }

    fn exec(&mut self, pgm: &[u8], op: u8) -> Result<()> {
        println!("exec: {}", op);
        match op {
            OP_TERM => self.running = false,
            OP_PUSH => {
                let v = self.load_vi64(pgm)?;
                self.push(Value::I64(v));
            },
            OP_ADD => {
                if let Value::I64(v1) = self.pop()? {
                    if let Value::I64(v2) = self.pop()? {
                        self.push(Value::I64(v1 + v2))?;
                        return Ok(())
                    }
                }
                return Err(RuntimeError::new("Invalid type for add"))
            }
            _ => (),
        }
        Ok(())
    }

    fn print_state(&self) {
        print!("pc:{} s:[", self.pc);
        for i in 0..self.sp {
            print!("{:?},", self.vstack[i]);
        }
        println!("]");
    }

    fn run1(&mut self, pgm: &[u8]) -> Result<()> {
        let op = self.load_u8(pgm)?;
        self.exec(pgm, op)?;
        Ok(())
    }

    fn run(&mut self, pgm: &[u8]) {
        //println!("Running pgm sized {}", size_of(pgm))
        self.pc = 0;
        self.running = true;
        self.print_state();
        while self.running {
            if let Err(err) = self.run1(pgm) {
                println!("Runtime Error: {}", err);
                self.running = false;
            }
            self.print_state();
        }
    }
}

pub fn store_vi64(v: i64) {

}

pub fn run() {
    let pgm: [u8;9] = [
        OP_PUSH, 0x12,
        OP_PUSH, 0x02,
        OP_PUSH, 0x07,
        OP_ADD,
        OP_ADD,
        OP_TERM];
    let mut vm = Vm::new();
    vm.run(&pgm);
}
