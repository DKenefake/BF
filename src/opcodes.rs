use crate::opcodes::Opcode::{JUMPIFNZERO, JUMPIFZERO, SCANBY};
use std::io::stdout;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Opcode {
    CHANGE { arg: i32 },
    MOVE { arg: i32 },
    PUTCHAR { arg: usize },
    READCHAR { arg: usize },
    JUMPIFZERO { arg: usize },
    JUMPIFNZERO { arg: usize },
    SETTO { arg: i32 },
    SCANBY { arg: i32 },
}

pub fn compile_code(program_code: String) -> Vec<Opcode> {
    let code = tokenize_instructions(program_code);

    println!("Program Tokens {:?}", code.len());

    let code = compress_foldable_opcodes(code);

    println!("Initial IR size {:?}", code.len());

    let code = remove_pointless_code_alteration(code);

    println!("Ineffective analysis size {:?}", code.len());

    let code = gen_scanning_ops(code);

    println!("Scan code analysis {:?}", code.len());

    let code = reset_bracket(code);

    let lll = find_lowest_level_loops(&code);

    for (x, y) in lll {
        println!("Lowest Level Loops at {x} {y}")
    }

    code
}

pub fn tokenize_instructions(program_code: String) -> Vec<Opcode> {
    let mut loop_stack = vec![];
    let mut ops = vec![];

    // remove anything that isn't a BF statement

    let valid_chars = vec!['+', '-', '<', '>', ',', '.', '[', ']'];

    let bf_valid_code = |c| valid_chars.contains(&c);

    let mut program_code = program_code.clone();
    program_code.retain(bf_valid_code);

    let mut pos = 0;

    // this is a good place as any to intercept the clear loop detection
    let program_code = program_code.replace("[-]", "C");
    let program_code = program_code.replace("[+]", "C");

    let program_chars = program_code.chars().collect::<Vec<char>>();

    while pos < program_chars.len() {
        let curr = program_chars[pos];

        let mut emit = |x| {
            ops.push(x);
            ops.len() - 1
        };

        match curr {
            '[' => {
                let ins_pos = emit(JUMPIFZERO { arg: 0 });
                loop_stack.push(ins_pos);
            }
            ']' => {
                let open_instr = loop_stack[loop_stack.len() - 1];
                loop_stack.pop();
                let close_instr = emit(JUMPIFNZERO { arg: open_instr });
                ops[open_instr] = Opcode::JUMPIFZERO { arg: close_instr };
            }
            '+' => {
                ops.push(Opcode::CHANGE { arg: 1 });
            }
            '-' => {
                ops.push(Opcode::CHANGE { arg: -1 });
            }
            '<' => {
                ops.push(Opcode::MOVE { arg: -1 });
            }
            '>' => {
                ops.push(Opcode::MOVE { arg: 1 });
            }
            '.' => {
                ops.push(Opcode::PUTCHAR { arg: 1 });
            }
            ',' => {
                ops.push(Opcode::READCHAR { arg: 1 });
            }
            'C' => {
                ops.push(Opcode::SETTO { arg: 0 });
            }
            _ => {}
        }

        pos += 1;
    }

    ops
}

pub fn compress_foldable_opcodes(opcodes: Vec<Opcode>) -> Vec<Opcode> {
    assert!(opcodes.len() > 0);

    let mut output = vec![*opcodes.first().unwrap()];

    for &opcode in opcodes.iter().skip(1) {
        match opcode {
            Opcode::CHANGE { arg: arg1 } => match output[output.len() - 1] {
                Opcode::CHANGE { arg: arg2 } => {
                    output.pop();
                    if arg1 + arg2 != 0 {
                        output.push(Opcode::CHANGE { arg: arg1 + arg2 });
                    }
                }
                _ => output.push(Opcode::CHANGE { arg: arg1 }),
            },
            Opcode::MOVE { arg: arg1 } => match output[output.len() - 1] {
                Opcode::MOVE { arg: arg2 } => {
                    output.pop();
                    if arg1 + arg2 != 0 {
                        output.push(Opcode::MOVE { arg: arg1 + arg2 });
                    }
                }
                _ => output.push(Opcode::MOVE { arg: arg1 }),
            },
            Opcode::PUTCHAR { arg: arg1 } => match output[output.len() - 1] {
                Opcode::PUTCHAR { arg: arg2 } => {
                    output.pop();
                    if arg1 + arg2 != 0 {
                        output.push(Opcode::PUTCHAR { arg: arg1 + arg2 });
                    }
                }
                _ => output.push(Opcode::PUTCHAR { arg: arg1 }),
            },
            Opcode::READCHAR { arg: arg1 } => match output[output.len() - 1] {
                Opcode::READCHAR { arg: arg2 } => {
                    output.pop();
                    if arg1 + arg2 != 0 {
                        output.push(Opcode::READCHAR { arg: arg1 + arg2 });
                    }
                }
                _ => output.push(Opcode::READCHAR { arg: arg1 }),
            },

            _ => output.push(opcode),
        }
    }

    output
}

pub fn remove_pointless_code_alteration(opcodes: Vec<Opcode>) -> Vec<Opcode> {
    // opcode sequences that have the following ... ,CHANGE(x),SetTo(x), ...
    // can be reduced to ..., SetTo(x), ..., we are clearing that location anyway

    // in the same way ..., SetTo(x), Change(k), is equivalent to the following
    // ..., SetTo(x + k),

    let mut output = vec![*opcodes.first().unwrap()];

    for &opcode in opcodes.iter().skip(1) {
        match opcode {
            Opcode::SETTO { .. } => match output[output.len() - 1] {
                Opcode::CHANGE { .. } => {
                    output.pop();
                    output.push(opcode)
                }
                _ => output.push(opcode),
            },

            Opcode::CHANGE { arg } => match output[output.len() - 1] {
                Opcode::SETTO { arg: arg2 } => {
                    output.pop();
                    output.push(Opcode::SETTO { arg: arg + arg2 })
                }
                _ => output.push(opcode),
            },

            _ => output.push(opcode),
        }
    }

    output
}

pub fn gen_scanning_ops(opcodes: Vec<Opcode>) -> Vec<Opcode> {
    // given the following code ..., JumpIfZero, Move(K), JumpifNotZero, ...
    // this can be replaced with the following opcodes ..., ScanBy(k), ....

    let mut output: Vec<Opcode> = vec![opcodes[0], opcodes[1]];

    for &opcode in opcodes.iter().skip(2) {
        match opcode {
            JUMPIFNZERO { .. } => match output[output.len() - 1] {
                Opcode::MOVE { arg } => {
                    if matches!(output[output.len() - 2], JUMPIFZERO { .. }) {
                        output.pop();
                        output.pop();
                        output.push(SCANBY { arg });
                    } else {
                        output.push(opcode)
                    }
                }
                _ => output.push(opcode),
            },
            _ => output.push(opcode),
        }
    }

    output
}

pub fn find_lowest_level_loops(opcodes: &Vec<Opcode>) -> Vec<(usize, usize)> {
    // basically scan from every open bracket and if we hit a closing bracket before
    // we hit another open bracket we know this is the lowest level possible which opens up some
    // possibilities for optimization

    let mut all_brackets = vec![];

    for &op in opcodes.iter() {
        match op {
            JUMPIFNZERO { .. } => all_brackets.push(op),
            JUMPIFZERO { .. } => all_brackets.push(op),
            _ => {}
        }
    }

    let mut output = vec![];

    for (&op1, &op2) in all_brackets.iter().zip(all_brackets.iter().skip(1)) {
        match (op1, op2) {
            (JUMPIFZERO { arg: x }, JUMPIFNZERO { arg: y }) => output.push((y, x)),
            _ => {}
        }
    }

    output
}

pub fn reset_bracket(opcodes: Vec<Opcode>) -> Vec<Opcode> {
    let mut pos = 0;
    let mut ops = vec![];
    let mut loop_stack = vec![];

    while pos < opcodes.len() {
        let curr = opcodes[pos];

        let mut emit = |x| {
            ops.push(x);
            ops.len() - 1
        };

        match curr {
            JUMPIFZERO { .. } => {
                let ins_pos = emit(JUMPIFZERO { arg: 0 });
                loop_stack.push(ins_pos);
            }
            JUMPIFNZERO { .. } => {
                let open_instr = loop_stack[loop_stack.len() - 1];
                loop_stack.pop();
                let close_instr = emit(JUMPIFNZERO { arg: open_instr });
                ops[open_instr] = Opcode::JUMPIFZERO { arg: close_instr };
            }
            _ => {
                ops.push(curr);
            }
        }

        pos += 1;
    }

    ops
}
