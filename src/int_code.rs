use std::collections::VecDeque;

enum Operation {
    Add(i32, i32, usize),
    Multiply(i32, i32, usize),
    Input(usize),
    Output(i32),
    JumpTrue(i32, i32),
    JumpFalse(i32, i32),
    CmpLess(i32, i32, usize),
    CmpEqual(i32, i32, usize),
    Halt,
}

enum Mode {
    Address,
    Intermediate,
}

struct ModeFlag {
    flag: i32,
}

impl ModeFlag {
    fn new(flag: i32) -> ModeFlag {
        ModeFlag { flag }
    }

    fn next(&mut self) -> Mode {
        let flag = (self.flag % 10) == 0;
        self.flag /= 10;
        if flag {
            Mode::Address
        } else {
            Mode::Intermediate
        }
    }
}

pub struct IntProgram {
    memory: Vec<i32>,
    pc: usize,
}

impl IntProgram {
    pub fn new(memory: Vec<i32>) -> IntProgram {
        IntProgram { memory, pc: 0 }
    }

    pub fn execute(&mut self, input: Vec<i32>) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        let mut input: VecDeque<i32> = VecDeque::from(input);
        loop {
            let op = self.extract_op();
            match op {
                Some(op) => match op {
                    Operation::Halt => break,
                    _ => self.iterate(op, &mut input, &mut output),
                },
                None => panic!("program error"),
            }
        }
        output
    }

    fn param(&mut self, mode: Mode) -> Option<i32> {
        match self.memory.get(self.pc) {
            Some(i) => {
                self.pc += 1;
                match mode {
                    Mode::Address => match self.memory.get(*i as usize) {
                        Some(j) => Some(*j),
                        None => None,
                    },
                    Mode::Intermediate => Some(*i),
                }
            }
            None => None,
        }
    }

    fn extract_op(&mut self) -> Option<Operation> {
        match self.param(Mode::Intermediate) {
            Some(op_code) => {
                let mut flag = ModeFlag::new(op_code / 100);
                match op_code % 100 {
                    1 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.param(Mode::Intermediate) {
                            Some(addr) => Some(Operation::Add(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    2 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.param(Mode::Intermediate) {
                            Some(addr) => Some(Operation::Multiply(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    3 => match self.param(Mode::Intermediate) {
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
                        (Some(op1), Some(op2)) => match self.param(Mode::Intermediate) {
                            Some(addr) => Some(Operation::CmpLess(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    8 => match (self.param(flag.next()), self.param(flag.next())) {
                        (Some(op1), Some(op2)) => match self.param(Mode::Intermediate) {
                            Some(addr) => Some(Operation::CmpEqual(op1, op2, addr as usize)),
                            None => None,
                        },
                        _ => None,
                    },
                    99 => Some(Operation::Halt),
                    _ => None,
                }
            }
            None => None,
        }
    }

    fn iterate(&mut self, op: Operation, input: &mut VecDeque<i32>, output: &mut Vec<i32>) {
        match op {
            Operation::Add(op1, op2, addr) => {
                if let Some(e) = self.memory.get_mut(addr) {
                    *e = op1 + op2;
                }
            }
            Operation::Multiply(op1, op2, addr) => {
                if let Some(e) = self.memory.get_mut(addr) {
                    *e = op1 * op2;
                }
            }
            Operation::Input(addr) => {
                let input = input.pop_front().expect("insufficient input");
                if let Some(e) = self.memory.get_mut(addr) {
                    *e = input;
                }
            }
            Operation::Output(op) => output.push(op),
            Operation::JumpTrue(op1, pc) => {
                if op1 != 0 {
                    self.pc = pc as usize
                }
            }
            Operation::JumpFalse(op1, pc) => {
                if op1 == 0 {
                    self.pc = pc as usize
                }
            }
            Operation::CmpLess(op1, op2, addr) => {
                if let Some(e) = self.memory.get_mut(addr) {
                    *e = if op1 < op2 { 1 } else { 0 };
                }
            }
            Operation::CmpEqual(op1, op2, addr) => {
                if let Some(e) = self.memory.get_mut(addr) {
                    *e = if op1 == op2 { 1 } else { 0 };
                }
            }
            Operation::Halt => (),
        };
    }
}
