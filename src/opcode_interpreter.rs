use crate::bf_executor::BFExecuter;
use crate::opcodes::Opcode;
use crate::state::ProgramState;
use std::io::{Read, Stdin, Stdout, Write};

pub struct BFOpcodeInterpreter {
    program: Vec<Opcode>,
    pub machine: ProgramState,
    stdin: Stdin,
    stdout: Stdout,
    inst_evaluated: usize,
}

impl BFOpcodeInterpreter {
    pub fn new(program_source: Vec<Opcode>) -> Self {
        Self {
            program: program_source,
            machine: ProgramState::new(),
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            inst_evaluated: 0,
        }
    }
}

impl BFExecuter for BFOpcodeInterpreter {
    fn execute(&mut self) {
        while self.machine.ip < self.program.len() {
            match self.program[self.machine.ip] {
                Opcode::CHANGE { arg } => {
                    let new_state = self.machine.memory[self.machine.dp] as i32 + arg;
                    self.machine.memory[self.machine.dp] = new_state as u8;
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
                    self.machine.memory[self.machine.dp] = arg as u8;
                }
                Opcode::SCANBY { arg } => {
                    while self.machine.memory[self.machine.dp] != 0 {
                        self.machine.dp = (self.machine.dp as i32 + arg) as usize;
                    }
                }
                Opcode::MULTI { arg1: x, arg2: y } => {
                    let placement_index = (self.machine.dp as i32 + x).max(0);
                    self.machine.memory[placement_index as usize] +=
                        y as u8 * self.machine.memory[self.machine.dp];
                }
                Opcode::MOVINGCHANGE {
                    arg1: x,
                    arg2: y,
                    arg3: z,
                } => {
                    while self.machine.memory[self.machine.dp] != 0 {
                        self.machine.dp = (self.machine.dp as i32 + x) as usize;
                        self.machine.memory[self.machine.dp] += y as u8;
                        self.machine.dp = (self.machine.dp as i32 + z) as usize;
                    }
                }
                Opcode::MOVINGSET {
                    arg1: x,
                    arg2: y,
                    arg3: z,
                } => {
                    while self.machine.memory[self.machine.dp] != 0 {
                        self.machine.dp = (self.machine.dp as i32 + x) as usize;
                        self.machine.memory[self.machine.dp] = y as u8;
                        self.machine.dp = (self.machine.dp as i32 + z) as usize;
                    }
                }
            }
            self.machine.ip += 1;
            self.inst_evaluated += 1;
        }

        // make sure we flush out everything in the
        std::io::stdout()
            .flush()
            .expect("Expected to be able to flush stdout");
    }

    fn read_char(&mut self) {
        let mut byte = [0_u8];
        self.stdin
            .lock()
            .read_exact(&mut byte)
            .expect("Expected to be able to read a single char");
        self.machine.memory[self.machine.dp] = byte[0];
    }

    fn write_char(&mut self) {
        let mut byte = [0_u8];
        byte[0] = (self.machine.memory[self.machine.dp] & 0xFF) as u8;
        self.stdout
            .write_all(&byte)
            .expect("Expected to be able to write a single char");
    }

    fn instruction_count(&mut self) -> usize {
        self.inst_evaluated
    }

    fn reset_machine_state(&mut self) {
        self.machine = ProgramState::new();
    }
}
