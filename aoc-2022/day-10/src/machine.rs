use std::{fmt::Display, vec::IntoIter};

use crate::prelude::*;

pub struct Machine {
    registers: Registers,
    ops: IntoIter<Op>,
    cur: Option<OpExec>,
}

impl Machine {
    pub fn new(ops: Vec<Op>) -> Self {
        let registers = Registers::new();
        let ops = ops.into_iter();
        let cur = None;
        Self {
            registers,
            cur,
            ops,
        }
    }
    // runs to completion, when there is no more work to be done
    pub fn run(&mut self) {
        if !self.load() {
            return;
        }
        for tick in (1 as u64).. {
            // load an operation if we don't have one yet
            if !self.load() {
                break;
            }

            println!("Tick: {tick}");

            // execute the current operation.
            if let Some(mut exec) = self.cur.take() {
                // if we still need to wait, return early
                if exec.dec_wait() {
                    self.cur = Some(exec);
                    continue;
                }
                // the exec is done.
                exec.apply(&mut self.registers);
            }
        }
    }
    // loads the next op if necessary, and returns true if we have an op
    fn load(&mut self) -> bool {
        if self.cur.is_none() {
            // there is no operation. load the next op.
            if let Some(op) = self.ops.next() {
                println!("Loaded: {op:?}");
                let cycles = op.cycles();
                let exec = OpExec { op, cycles };
                self.cur = Some(exec);
            }
        }
        !self.cur.is_none()
    }
}

struct OpExec {
    op: Op,
    cycles: i32,
}

impl OpExec {
    fn dec_wait(&mut self) -> bool {
        self.cycles -= 1;
        self.cycles > 0
    }
    fn apply(&self, registers: &mut Registers) {
        self.op.apply(registers);
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.registers)
    }
}
