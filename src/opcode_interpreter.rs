use crate::bf_executor::BFExecuter;
use crate::opcodes::Opcode;
use crate::state::ProgramState;
use std::io::{Read, Stdin, Stdout, Write};

pub struct BFOpcodeInterpreter {
    program: Vec<Opcode>,
    machine: ProgramState,
    stdin: Stdin,
    stdout: Stdout,
    inst_evaluated: usize,
}

impl BFOpcodeInterpreter {
    pub fn new(
        program_source: String,
        opcode_generator: fn(String) -> Vec<Opcode>,
    ) -> BFOpcodeInterpreter {
        let opcodes = opcode_generator(program_source);

        // for (i, op) in opcodes.iter().enumerate() {
        //     println!("{:?} {:?}", i, op);
        // }

        BFOpcodeInterpreter {
            program: opcodes,
            machine: ProgramState::new(),
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            inst_evaluated: 0,
        }
    }
}

impl BFExecuter for BFOpcodeInterpreter {
    fn execute(&mut self) -> () {
        while self.machine.ip < self.program.len() {
            match self.program[self.machine.ip] {
                Opcode::CHANGE { arg } => {
                    self.machine.memory[self.machine.dp] += arg;
                }
                Opcode::MOVE { arg } => {
                    self.machine.dp = (self.machine.dp as i32 + arg) as usize;
                }
                Opcode::PUTCHAR { arg } => {
                    for _ in 0..arg {
                        self.write_char();
                    }
                }
                Opcode::READCHAR { arg } => {
                    for _ in 0..arg {
                        self.read_char();
                    }
                }
                Opcode::JUMPIFZERO { arg } => {
                    if self.machine.memory[self.machine.dp] == 0 {
                        self.machine.ip = arg;
                    }
                }
                Opcode::JUMPIFNZERO { arg } => {
                    if self.machine.memory[self.machine.dp] != 0 {
                        self.machine.ip = arg;
                    }
                }
                Opcode::SETTO { arg } => {
                    self.machine.memory[self.machine.dp] = arg;
                }
                Opcode::SCANBY { arg } => {
                    while self.machine.memory[self.machine.dp] != 0 {
                        self.machine.dp = (self.machine.dp as i32 + arg) as usize;
                    }
                }
                Opcode::Multi { arg1: x, arg2: y } => {
                    let mut placement_index = (self.machine.dp as i32 + x).max(0);
                    self.machine.memory[placement_index as usize] +=
                        y * self.machine.memory[self.machine.dp];
                }
            }
            self.machine.ip += 1;
            self.inst_evaluated += 1;
        }
    }

    fn read_char(&mut self) -> () {
        let mut byte = [0_u8];
        self.stdin.lock().read_exact(&mut byte).expect("TODO");
        self.machine.memory[self.machine.dp] = byte[0] as i32;
    }

    fn write_char(&mut self) -> () {
        let mut byte = [0_u8];
        byte[0] = self.machine.memory[self.machine.dp] as u8;
        self.stdout.write(&byte).expect("TODO: panic message");
    }

    fn instruction_count(&mut self) -> usize {
        self.inst_evaluated
    }
}
