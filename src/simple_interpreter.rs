use crate::bf_executor::BFExecuter;
use crate::state::ProgramState;
use std::io::{Read, Stdin, Stdout, Write};

pub struct BFSimpleInterpreter {
    program: Vec<char>,
    machine: ProgramState,
    stdin: Stdin,
    stdout: Stdout,
    inst_evaluated: usize,
}

impl BFSimpleInterpreter {
    pub fn new(program: String) -> BFSimpleInterpreter {
        BFSimpleInterpreter {
            program: program.chars().collect(),
            machine: ProgramState::new(),
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            inst_evaluated: 0,
        }
    }
}

impl BFExecuter for BFSimpleInterpreter {
    fn execute(&mut self) {
        while self.machine.ip < self.program.len() {
            let curr = self.program[self.machine.ip];

            match curr {
                '+' => {
                    self.machine.memory[self.machine.dp] += 1;
                }
                '-' => {
                    self.machine.memory[self.machine.dp] -= 1;
                }
                '>' => {
                    if !self.machine.is_valid_dp_location(self.machine.dp + 1) {
                        println!(
                            "Encountered Illegal Data Pointer location at Instruction {:?}",
                            self.machine.ip
                        );
                        return;
                    }
                    self.machine.dp += 1;
                }
                '<' => {
                    if !self.machine.is_valid_dp_location(self.machine.dp - 1) {
                        println!(
                            "Encountered Illegal Data Pointer location at Instruction {:?}",
                            self.machine.ip
                        );
                        return;
                    }
                    self.machine.dp -= 1;
                }
                ',' => {
                    self.read_char();
                }
                '.' => {
                    self.write_char();
                }
                '[' => {
                    if self.machine.memory[self.machine.dp] == 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            self.machine.ip += 1;
                            match *self.program.get(self.machine.ip).unwrap() {
                                '[' => {
                                    depth += 1;
                                }
                                ']' => {
                                    depth -= 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                ']' => {
                    if self.machine.memory[self.machine.dp] != 0 {
                        let mut depth = 1;
                        while depth != 0 {
                            self.machine.ip -= 1;
                            match *self.program.get(self.machine.ip).unwrap() {
                                ']' => {
                                    depth += 1;
                                }
                                '[' => {
                                    depth -= 1;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }

            self.machine.ip += 1;
            self.inst_evaluated += 1;
        }
    }

    fn read_char(&mut self) {
        let mut byte = [0_u8];
        self.stdin.lock().read_exact(&mut byte).expect("TODO");
        self.machine.memory[self.machine.dp] = byte[0] as i32;
    }

    fn write_char(&mut self) {
        let mut byte = [0_u8];
        byte[0] = self.machine.memory[self.machine.dp] as u8;
        self.stdout.write_all(&byte).expect("Wrote out the char correctly");
    }

    fn instruction_count(&mut self) -> usize {
        self.inst_evaluated
    }

    fn reset_machine_state(&mut self) {
        self.machine = ProgramState::new();
    }
}
