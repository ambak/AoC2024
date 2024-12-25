#[macro_use]
extern crate scan_fmt;
use std::io::{self, BufRead};

struct CPU3bit {
    a: i64,
    b: i64,
    c: i64,
    v: Vec<i64>,
    output: Vec<i64>,
    i: i64,
}
impl CPU3bit {
    pub fn new() -> CPU3bit {
        CPU3bit {
            a: 0,
            b: 0,
            c: 0,
            v: vec![],
            output: vec![],
            i: 0,
        }
    }
    fn adv(&mut self, op: i64) {
        self.a >>= self.combo(op);
    }
    fn bxl(&mut self, op: i64) {
        self.b ^= op;
    }
    fn bst(&mut self, op: i64) {
        self.b = self.combo(op) % 8;
    }
    fn jnz(&mut self, op: i64) {
        if self.a != 0 {
            self.i = op - 2;
        }
    }
    fn bxc(&mut self, _op: i64) {
        self.b ^= self.c;
    }
    fn out(&mut self, op: i64) {
        let x = self.combo(op) % 8;
        self.output.push(x);
    }
    fn bdv(&mut self, op: i64) {
        self.b = self.a >> self.combo(op);
    }
    fn cdv(&mut self, op: i64) {
        self.c = self.a >> self.combo(op);
    }
    pub fn run(&mut self) {
        let mut f0 = CPU3bit::adv;
        let mut f1 = CPU3bit::bxl;
        let mut f2 = CPU3bit::bst;
        let mut f3 = CPU3bit::jnz;
        let mut f4 = CPU3bit::bxc;
        let mut f5 = CPU3bit::out;
        let mut f6 = CPU3bit::bdv;
        let mut f7 = CPU3bit::cdv;
        let mut f: Vec<&mut dyn FnMut(&mut Self, i64)> = vec![
            &mut f0, &mut f1, &mut f2, &mut f3, &mut f4, &mut f5, &mut f6, &mut f7,
        ];
        while self.i >= 0 && self.i + 1 < self.v.len() as i64 {
            f[self.v[self.i as usize] as usize](self, self.v[(self.i + 1) as usize]);
            self.i += 2;
        }
    }
    pub fn print(&self) {
        for j in 0..self.output.len() - 1 {
            print!("{},", self.output[j]);
        }
        print!("{}\n", self.output[self.output.len() - 1]);
    }
    fn combo(&mut self, op: i64) -> i64 {
        if op <= 3 {
            return op;
        }
        if op == 4 {
            return self.a;
        }
        if op == 5 {
            return self.b;
        }
        if op == 6 {
            return self.c;
        }
        panic!();
    }
}

fn main() {
    let lines = io::stdin().lock().lines();

    let mut cpu = CPU3bit::new();
    for line in lines {
        let l = line.unwrap();
        if let Ok(val) = scan_fmt!(&l, "Register A: {}", i64) {
            cpu.a = val;
        } else if let Ok(val) = scan_fmt!(&l, "Register B: {}", i64) {
            cpu.b = val;
        } else if let Ok(val) = scan_fmt!(&l, "Register C: {}", i64) {
            cpu.c = val;
        } else if let Ok(val) = scan_fmt!(&l, "Program: {}", String) {
            for x in val.split(',') {
                cpu.v.push(x.parse::<i64>().unwrap());
            }
        }
    }
    cpu.run();
    cpu.print();
}
