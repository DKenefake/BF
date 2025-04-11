use bf::bf_executor::BFExecuter;
use bf::opcode_interpreter::BFOpcodeInterpreter;
use bf::opcodes::compile_code;
use bf::simple_interpreter::BFSimpleInterpreter;
use bf::source_utils::{check_program_brackets, remove_no_coding_symbols};
use std::io::Write;
use std::process::exit;

enum Action {
    Exit,
    UseDebugInterpreter,
    UseFastInterpreter,
}

enum InterpreterMode {
    UseDebugInterpreter,
    UseFastInterpreter,
}

fn prompt_repl_input() -> String {
    print!(":: ");
    std::io::stdout().flush().expect("Flush stdout failed");
    let mut input = String::new();
    let res = std::io::stdin().read_line(&mut input);

    match res {
        Ok(_) => {
            input.retain(|c| !c.is_whitespace());
            input
        }
        Err(_) => String::from(""),
    }
}

fn find_action(input: &str) -> Option<Action> {
    // our magic words are not case-sensitive!
    let lowered_input = input.to_owned().to_lowercase();

    if lowered_input.contains("exit") {
        return Some(Action::Exit);
    }

    if lowered_input.contains("debug") {
        return Some(Action::UseDebugInterpreter);
    }

    if lowered_input.contains("fast") {
        return Some(Action::UseFastInterpreter);
    }

    None
}

fn main() {
    // make REPL Entry Message
    println!("Welcome to the BF Repl!");
    println!("All lines are run on a fresh interpreter");
    println!("Please enter your program as a single line");
    println!("Some magic actions");
    println!("entering exit will exit the program");
    println!("entering debug will switch to debugging mode (default!)");
    println!("entering fast will switch to the fast interpreter");

    let mut interpreter_mode = InterpreterMode::UseDebugInterpreter;

    loop {
        let input = prompt_repl_input();
        let possible_action = find_action(&input);

        if let Some(action) = possible_action {
            match action {
                Action::Exit => {
                    exit(1);
                }
                Action::UseDebugInterpreter => {
                    interpreter_mode = InterpreterMode::UseDebugInterpreter;
                }
                Action::UseFastInterpreter => {
                    interpreter_mode = InterpreterMode::UseFastInterpreter;
                }
            }
            continue;
        } else {
            let sanitized_code = remove_no_coding_symbols(input);

            if sanitized_code.is_empty() {
                println!("There is no BF code here...");
                continue;
            }

            if !check_program_brackets(&sanitized_code) {
                println!("This is not valid BF code! Check your brackets!");
                continue;
            }

            match interpreter_mode {
                InterpreterMode::UseDebugInterpreter => {
                    let compiled_code = compile_code(sanitized_code);
                    BFOpcodeInterpreter::new(compiled_code).execute()
                }
                InterpreterMode::UseFastInterpreter => {
                    BFSimpleInterpreter::new(sanitized_code).execute()
                }
            };
        }
    }
}
