use std::collections::VecDeque;

#[derive(Debug)]
pub enum Operation {
    Add(i64, i64, usize),
    Multiply(i64, i64, usize),
    Input(usize),
    Output(i64),
    JumpTrue(i64, i64),
    JumpFalse(i64, i64),
    CmpLess(i64, i64, usize),
    CmpEqual(i64, i64, usize),
    Halt,
    Rebase(i64),
}

impl Operation {
    fn size(&self) -> usize {
        match self {
            Operation::Add(_, _, _) => 4,
            Operation::Multiply(_, _, _) => 4,
            Operation::Input(_) => 2,
            Operation::Output(_) => 2,
            Operation::JumpFalse(_, _) => 3,
            Operation::JumpTrue(_, _) => 3,
            Operation::CmpLess(_, _, _) => 4,
            Operation::CmpEqual(_, _, _) => 4,
            Operation::Halt => 1,
            Operation::Rebase(_) => 2,
        }
    }
}

enum Mode {
    Address,
    Intermediate,
    Relative,
}

pub trait Feeder {
    fn next(&mut self) -> Option<i64>;
}

impl Feeder for VecDeque<i64> {
    fn next(&mut self) -> Option<i64> {
        self.pop_front()
    }
}

struct ModeFlag {
    flag: i64,
}

impl ModeFlag {
    fn new(flag: i64) -> ModeFlag {
        ModeFlag { flag }
    }

    fn next(&mut self) -> Mode {
        let flag = self.flag % 10;
        self.flag /= 10;
        match flag {
            0 => Mode::Address,
            1 => Mode::Intermediate,
            _ => Mode::Relative,
        }
    }
}

#[derive(Clone)]
pub struct IntProgram {
    memory: Vec<i64>,
    pc: usize,
    halt: bool,
    base: i64,
}

impl IntProgram {
    pub fn new(memory: Vec<i64>) -> IntProgram {
        IntProgram { memory, pc: 0, halt: false, base: 0 }
    }

    pub fn can_run(&self) -> bool {
        !self.halt
    }

    pub fn execute(&mut self, input: Vec<i64>) -> Vec<i64> {
        let mut output = Vec::new();
        let mut input = VecDeque::from(input);
        while self.can_run() {
            let op = self.extract_op();
            match op {
                Some(op) => {
                    let size = op.size();
                    match self.iterate(op, &mut input) {
                        (true, Some(out)) => output.push(out),
                        (false, _) => {
                            self.pc -= size;
                            break;
                        }
                        _ => (),
                    }
                }
                None => {
                    println!("extraction failed");
                    break;
                }
            }
        }
        output
    }

    pub fn get(&mut self, index: usize) -> i64 {
        while self.memory.len() <= index {
            self.memory.push(0)
        }
        *self.memory.get(index).expect("push failed")
    }

    pub fn set(&mut self, index: usize, val: i64) {
        while self.memory.len() <= index {
            self.memory.push(0)
        }
        *self.memory.get_mut(index).expect("push failed") = val;
    }

    fn param(&mut self, mode: Mode) -> Option<i64> {
        let immediate_value = self.get(self.pc);
        self.pc += 1;
        match mode {
            Mode::Address => {
                if immediate_value < 0 {
                    None
                } else {
                    Some(self.get(immediate_value as usize))
                }
            }
            Mode::Intermediate => Some(immediate_value),
            Mode::Relative => {
                let addr = immediate_value + self.base;
                if addr < 0 {
                    None
                } else {
                    Some(self.get(addr as usize))
                }
            }
        }
    }

    fn address(&mut self, mode: Mode) -> Option<usize> {
        let immediate_value = self.get(self.pc);
        self.pc += 1;
        match mode {
            Mode::Address => {
                if immediate_value < 0 {
                    None
                } else {
                    Some(immediate_value as usize)
                }
            }
            Mode::Relative => {
                let addr = immediate_value + self.base;
                if addr < 0 {
                    None
                } else {
                    Some(addr as usize)
                }
            }
            _ => None,
        }
    }

    pub fn extract_op(&mut self) -> Option<Operation> {
        match self.param(Mode::Intermediate) {
            Some(op_code) => {
                let mut flag = ModeFlag::new(op_code / 100);
                match op_code % 100 {
                    1 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.address(flag.next()) {
                            Some(addr) => Some(Operation::Add(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    2 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.address(flag.next()) {
                            Some(addr) => Some(Operation::Multiply(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    3 => match self.address(flag.next()) {
                        Some(i) => Some(Operation::Input(i as usize)),
                        None => None,
                    },
                    4 => match self.param(flag.next()) {
                        Some(i) => Some(Operation::Output(i)),
                        None => None,
                    },
                    5 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => Some(Operation::JumpTrue(op1, op2)),
                        _ => None,
                    },
                    6 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => Some(Operation::JumpFalse(op1, op2)),
                        _ => None,
                    },
                    7 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.address(flag.next()) {
                            Some(addr) => Some(Operation::CmpLess(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    8 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.address(flag.next()) {
                            Some(addr) => Some(Operation::CmpEqual(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    99 => Some(Operation::Halt),
                    9 => match self.param(flag.next()) {
                        Some(i) => Some(Operation::Rebase(i)),
                        None => None,
                    },
                    _ => None,
                }
            }
            None => None,
        }
    }

    pub fn iterate(&mut self, op: Operation, input: &mut dyn Feeder) -> (bool, Option<i64>) {
        match op {
            Operation::Add(op1, op2, addr) => {
                self.set(addr, op1 + op2);
            }
            Operation::Multiply(op1, op2, addr) => {
                self.set(addr, op1 * op2);
            }
            Operation::Input(addr) => {
                if let Some(i) = input.next() {
                    self.set(addr, i);
                } else {
                    return (false, None);
                }
            }
            Operation::Output(op1) => return (true, Some(op1)),
            Operation::JumpTrue(op1, pc) => {
                if op1 != 0 {
                    self.pc = pc as usize;
                }
            }
            Operation::JumpFalse(op1, pc) => {
                if op1 == 0 {
                    self.pc = pc as usize;
                }
            }
            Operation::CmpLess(op1, op2, addr) => {
                self.set(addr, if op1 < op2 { 1 } else { 0 });
            }
            Operation::CmpEqual(op1, op2, addr) => {
                self.set(addr, if op1 == op2 { 1 } else { 0 });
            }
            Operation::Halt => {
                self.halt = true;
            }
            Operation::Rebase(op1) => {
                self.base += op1;
            }
        }
        (true, None)
    }
}
