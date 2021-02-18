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

const OP_P0: u8 = 0x00;
const OP_P1: u8 = 0x10;
const OP_P2: u8 = 0x40;
const OP_P3: u8 = 0x80;
const OP_P4: u8 = 0xa0;

// P0 - None
const OP_TERM: u8 = 0x00;
// P1 - OpA
const OP_LOGV: u8 = 0x10;
// P2 - OpA, OpB
// P3 - OpA, OpB, OpC
const OP_ADD: u8 = 0x80;
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

pub struct Vm {
//    res: Rc<Res>,
    local: [Value; 16],
    pc: usize,
    running: bool,
}

// 0x20 0x01 0xab
impl Vm {
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

    fn load(&mut self, pgm: &[u8]) -> Result<Inst> {
        let op = self.load_u8(pgm)?;
        Ok(if op >= OP_P4 {
            Inst{
                op,
                op_a: self.load_u8(pgm)?,
                op_b: 0,
                op_c: 0,
                im: self.load_vi64(pgm)?,
            }
        } else if op >= OP_P3 {
            Inst{
                op,
                op_a: self.load_u8(pgm)?,
                op_b: self.load_u8(pgm)?,
                op_c: self.load_u8(pgm)?,
                im: 0,
            }
        } else if op >= OP_P2 {
            Inst{
                op,
                op_a: self.load_u8(pgm)?,
                op_b: self.load_u8(pgm)?,
                op_c: 0,
                im: 0,
            }
        } else if op >= OP_P1 {
            Inst{
                op,
                op_a: self.load_u8(pgm)?,
                op_b: 0,
                op_c: 0,
                im: 0,
            }
        } else {
            Inst{
                op,
                op_a: 0,
                op_b: 0,
                op_c: 0,
                im: 0,
            }
        })
    }

    fn local_i64(&self, n: u8) -> i64 {
        if let Value::I64(i) = self.local[n as usize] {
            i
        } else {
            0
        }
    }

    fn exec(&mut self, inst: Inst) {
        println!("exec: {}", inst.op);
        match inst.op {
            OP_TERM => self.running = false,
            OP_LOGV => println!("local[{}] = {:?}", inst.op_a, self.local[usize::from(inst.op_a)]),
            OP_ADDI => {
                if let Value::I64(i) = self.local[usize::from(inst.op_a)] {
                    self.local[usize::from(inst.op_a)] = Value::I64(i + inst.im);
                }
            },
            OP_ADD => {
                self.local[usize::from(inst.op_a)] = Value::I64(self.local_i64(inst.op_b) + self.local_i64(inst.op_c))
            }
            OP_LOADI => self.local[usize::from(inst.op_a)] = Value::I64(inst.im),
            _ => (),
        }
    }

    fn run(&mut self, pgm: &[u8]) {
        //println!("Running pgm sized {}", size_of(pgm))
        self.pc = 0;
        self.running = true;
        while self.running {
            match self.load(pgm) {
                Ok(inst) => self.exec(inst),
                Err(err) => {
                    println!("Runtime Error: {}", err);
                    self.running = false;
                }
            }
        }
    }
}

pub fn store_vi64(v: i64) {

}

pub fn run() {
    let pgm: [u8;23] = [OP_LOGV,0xf,
        OP_LOADI, 0x0f,0x11, OP_LOGV,0xf, OP_ADDI, 0xf, 0xff, 0x7f, OP_LOGV, 0x0f,
        OP_LOADI, 0x0, 0x10, OP_ADD, 0x1, 0x0, 0xf, OP_LOGV, 0x1,
        OP_TERM];
    let mut vm = Vm{
  //      res: Rc::new(Res {}),
        local: [Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void, Value::Void,],
        pc: 0,
        running: false,
    };
    vm.run(&pgm);
}
