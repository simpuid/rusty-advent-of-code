extern crate aoc;

use aoc::int_code::{Feeder, IntProgram};
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
struct Packet {
    address: usize,
    x: i64,
    y: i64,
}

impl Packet {
    fn new(address: usize, x: i64, y: i64) -> Packet {
        Packet { address, x, y }
    }
}

struct Computer {
    program: IntProgram,
    input: Buffer,
    output: Buffer,
}

struct Buffer {
    internal_buffer: VecDeque<i64>,
    idle: bool,
}

impl Buffer {
    fn push(&mut self, p: Packet) {
        self.internal_buffer.push_back(p.x);
        self.internal_buffer.push_back(p.y);
        self.idle = false;
    }

    fn pop(&mut self) -> Option<Packet> {
        if self.internal_buffer.len() >= 3 {
            if let (Some(address), Some(x), Some(y)) = (self.internal_buffer.pop_front(), self.internal_buffer.pop_front(), self.internal_buffer.pop_front()) {
                return Some(Packet::new(address as usize, x, y));
            }
        }
        None
    }

    fn feed(&mut self, x: i64) {
        self.internal_buffer.push_back(x);
    }
}

impl Feeder for Buffer {
    fn next(&mut self) -> Option<i64> {
        if let Some(e) = self.internal_buffer.pop_front() {
            self.idle = false;
            Some(e)
        } else {
            self.idle = true;
            Some(-1)
        }
    }
}

impl Computer {
    fn iterate(&mut self) -> Option<Packet> {
        if self.program.can_run() {
            if let Some(op) = self.program.extract_op() {
                if let (_, Some(e)) = self.program.iterate(op, &mut self.input) {
                    self.output.feed(e);
                }
            }
        }
        self.output.pop()
    }

    fn idle(&self) -> bool {
        self.input.idle
    }
}

fn construct_computers(count: usize, code: &[i64]) -> Vec<Computer> {
    let mut comps: Vec<Computer> = Vec::new();
    for i in 0..count {
        comps.push(Computer {
            program: IntProgram::new(code.to_vec()),
            input: Buffer {
                internal_buffer: VecDeque::from(vec![i as i64]),
                idle: false,
            },
            output: Buffer {
                internal_buffer: VecDeque::new(),
                idle: false,
            },
        });
    }
    comps
}

fn main() {
    let code = aoc::parse_file::<i64>("input.txt", ",");
    let mut comps = construct_computers(50, &code);
    let mut nat_packet: Option<Packet> = None;
    let mut last_y: Option<i64> = None;
    'main: loop {
        let mut packets: VecDeque<Packet> = VecDeque::new();
        for com in comps.iter_mut() {
            if let Some(p) = com.iterate() {
                packets.push_back(p);
            }
        }
        while let Some(p) = packets.pop_front() {
            if p.address == 255 {
                if nat_packet.is_none() {
                    println!("first packet to 255 {:?}", p);
                }
                nat_packet = Some(p);
            }
            if let Some(com) = comps.get_mut(p.address) {
                com.input.push(p);
            }
        }
        if let Some(p) = nat_packet {
            let mut idle_status = true;
            for com in comps.iter() {
                idle_status = idle_status && com.idle();
            }
            if idle_status {
                if let Some(c) = comps.get_mut(0) {
                    if let Some(e) = last_y {
                        if e == p.y {
                            println!("repeated y {}", p.y);
                            break 'main;
                        }
                    }
                    last_y = Some(p.y);
                    c.input.push(p);
                }
            }
        }
    }
}
