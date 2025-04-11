use bf::bf_executor::BFExecuter;
use bf::opcode_interpreter::BFOpcodeInterpreter;
use bf::opcodes::compile_code;
use bf::source_utils::{check_program_brackets, remove_no_coding_symbols};
use std::path::Path;
use std::process::exit;
use std::{fs, time};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "Program that will compile and run a BrainFuck program to an Intermediate Representation", long_about = None)]
struct Args{
    #[arg(short, long)]
    bf_file_path: String,
    #[arg(short = 'o', long, default_value = "false")]
    gen_object_file: bool
}

fn main() {

    let args = Args::parse();

    let program_path_str = args.bf_file_path;
    let _ = args.gen_object_file;

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

    let compiled_code = compile_code(program_source);



    let mut bf_inter = BFOpcodeInterpreter::new(compiled_code);

    let start_time = time::Instant::now();

    bf_inter.execute();

    let end_time = time::Instant::now();

    let program_run_time = end_time.duration_since(start_time).as_secs_f64();

    println!("Instructions {:?}", bf_inter.instruction_count());
    println!("Time Elapsed {:?} sec.", program_run_time);
}
