use std::fs;
use std::path::Path;
use std::process::exit;
use clap::Parser;
use bf::opcodes::Opcode;
use bf::source_utils::{check_program_brackets, get_c_file_prefix, remove_no_coding_symbols};

#[derive(Parser, Debug)]
#[command(version, about = "Program that will transcompile a BF program to C.", long_about = None)]
struct Args{
    #[arg(short, long)]
    bf_file_path: String,
    #[arg(short = 'o', long, default_value = "a.c")]
    c_output_name: String,
}


fn make_opcode_string(op:Opcode) -> String{
    match op {
        Opcode::CHANGE { arg } => {
            String::from(format!("change(m, p, {});\n", arg))
        }
        Opcode::MOVE { arg } => {
            String::from(format!("p = move(p, {});\n", arg))
        }
        Opcode::PUTCHAR { arg } => {
            String::from("display_char(m, p);\n".to_string()).repeat(arg)
        }
        Opcode::READCHAR { arg } => {
            String::from("m[p] = read_char();\n".to_string()).repeat(arg)
        }
        Opcode::JUMPIFZERO { .. } => {
            String::from("while (m[p] != 0){\n")
        }
        Opcode::JUMPIFNZERO { .. } => {
            String::from("}\n")
        }
        Opcode::SETTO { arg } => {
            String::from(format!("set_to(m, p, {});\n", arg))
        }
        Opcode::SCANBY { arg } => {
            String::from(format!("p = scan_by(m, p, {});\n", arg))
        }
        Opcode::MULTI { arg1, arg2 } => {
            String::from(format!("multi(m, p, {}, {});\n", arg1, arg2))
        }
        Opcode::MOVINGCHANGE { arg1, arg2, arg3 } => {
            String::from(format!("moving_change(m, p, {}, {}, {});\n", arg1, arg2, arg3))
        }
        Opcode::MOVINGSET { arg1, arg2, arg3 } => {
            String::from(format!("moving_set(m, p, {}, {}, {});\n", arg1, arg2, arg3))
        }
    }
}


fn main() {

    let args = Args::parse();

    let program_path_str = args.bf_file_path;
    let c_output_path_str = args.c_output_name;

    // generate the bf program from path
    let program_path = Path::new(&program_path_str);

    let program_source_result = fs::read_to_string(program_path);

    let program_source = match program_source_result {
        Ok(x) => remove_no_coding_symbols(x),
        Err(_) => panic!("File at path {:?} does not exist", program_path_str),
    };

    if !check_program_brackets(&program_source) {
        println!("This program is not valid incorrect number of brackets opening and closing!");
        exit(-1);
    }

    let compiled_code = bf::opcodes::compile_code(program_source);

    let mut c_code_output = vec![];

    let c_code_preamble = get_c_file_prefix();
    let c_code_postamble = String::from("return 0;\n}\n");
    let c_code_ops = compiled_code.iter().map(|x| make_opcode_string(*x)).collect::<Vec<_>>();

    c_code_output.push(c_code_preamble);
    c_code_output.extend(c_code_ops);
    c_code_output.push(c_code_postamble);

    let c_output_path = Path::new(&c_output_path_str);

    let c_code_full = c_code_output.join("");

    // write the c code to the file
    let c_code_output_result = fs::write(c_output_path, c_code_full);

    match c_code_output_result {
        Ok(_) => println!("C code written to {:?}", c_output_path_str),
        Err(_) => panic!("Failed to write C code to {:?}", c_output_path_str),
    }
}
