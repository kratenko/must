mod vm;

use std::fs;
use std::io::Error;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use failure::_core::cell::RefMut;

// "/base/entity"
// "/item/key"
type NatureId = u32;
type ResId = u32;

#[derive(Debug, Clone)]
enum Value {
    Void,
    I64(i64),
    F64(f64),
    String(Rc<String>),
    //Res(Rc<Res>),
}

fn h_trace(res: &Res) -> Value {
    println!("TRACE");
    Value::Void
}

fn h_get(res: &Res) -> Value {
    //Value::I32(*res.values.borrow().get(0).unwrap())
    println!("{:?}", res.values.borrow());
    //*res.values.borrow()
    Value::Void
}

fn h_inc(res: &Res) -> Value {
    res.mut_value(0, |val|{
        match val {
            Value::I64(ref mut i) => *i += 1,
            _ => (),
        };
    });
    Value::Void
}

struct Meth {
    h: fn(&Res) -> Value,
}

impl Meth {
    fn exec(&self, res: &Res) -> Value {
        (self.h)(res)
    }
}

struct Nature {
    pgm: Box<[u8]>,
    path: String,
    id: NatureId,
    base: Vec<Rc<Nature>>,
    methods: HashMap<String, Meth>,
}

impl Drop for Nature {
    fn drop(&mut self) {
        println!("Dropping n:{}#{}", self.path, self.id)
    }
}

struct Res {
    id: ResId,
    nature: Rc<Nature>,
//    environment_id: ResId,
    //properties: HashMap<String, i32>,
    values: RefCell<Box<[Value]>>,
}

impl Drop for Res {
    fn drop(&mut self) {
        println!("Dropping r:{}#{}", self.nature.path, self.id)
    }
}

impl Res {
    fn clone_value(&self, index: usize) -> Option<Value> {
        let vals = &mut **self.values.borrow_mut();
        if let Some(val) = vals.get_mut(index) {
            Some(val.clone())
        } else {
            None
        }
    }
    fn mut_value(&self, index: usize, h: fn (&mut Value)) {
        let vals = &mut **self.values.borrow_mut();
        if let Some(val) = vals.get_mut(index) {
            h(val);
        }
    }
    fn call(&mut self, fname: &str) -> Value {
        if let Some(m) = self.nature.methods.get(fname) {
            m.exec(&self)
        } else {
            println!("Unknown method");
            Value::Void
        }
    }
}

struct Registry {
    last_nature_id: NatureId,
    natures: HashMap<String, Rc<Nature>>,
    last_res_id: ResId,
    res: HashMap<ResId, Res>,
}

impl Registry {
    fn new() -> Registry {
        Registry{
            last_nature_id: 0,
            natures: Default::default(),
            last_res_id: 0,
            res: Default::default()
        }
    }

    fn generate_nature_id(&mut self) -> NatureId {
        self.last_nature_id += 1;
        self.last_nature_id
    }

    fn generate_res_id(&mut self) -> ResId {
        self.last_res_id += 1;
        self.last_res_id
    }

    fn load(&mut self, path: &str) -> Result<Rc<Nature>, String> {
        if let Some(nat) = self.natures.get(path) {
            return Ok(Rc::clone(&nat));
        }
        let mut methods = HashMap::new();
        methods.insert(String::from("trace"), Meth{ h: h_trace });
        methods.insert(String::from("get"), Meth{ h: h_get });
        methods.insert(String::from("inc"), Meth{ h: h_inc });
        let mut nat = Rc::new(Nature{
            pgm: Box::new([0x20,0x0f,0x11]),
            path: path.to_string(),
            id: self.generate_nature_id(),
            base: vec![],
            methods: methods,
        });
        self.natures.insert(path.to_string(), Rc::clone(&nat));
        Ok(nat)
    }

    //fn clone(&mut self, path: &str) -> Result<Rc<Res>, String> {
//
  //  }
}

fn main() {
    let mut reg = Registry::new();
    let n1 = reg.load("/dies").unwrap();
    let n2 = reg.load("/das").unwrap();
    let n3 = reg.load("/jenes").unwrap();

    let mut r1 = Res{
        id: 0,
        nature: n1.clone(),
        values: RefCell::new(Box::new([Value::I64(11)]))
    };
    r1.call("trace");
    println!("{:?}", r1.call("get"));
    println!("{:?}", r1.call("inc"));
    println!("{:?}", r1.call("get"));
    println!("---");
    vm::run();
    println!("---");
}
