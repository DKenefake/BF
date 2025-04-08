mod bf_executor;
mod simple_interpreter;
mod state;

use crate::bf_executor::BFExecuter;
use crate::simple_interpreter::BFSimpleInterpreter;
use std::path::Path;
use std::{env, fs, time};

fn main() {
    // generate the bf program path
    let program_path_str = env::args().collect::<Vec<String>>()[1].clone();
    let program_path = Path::new(&program_path_str);

    let program_source = fs::read_to_string(program_path).expect("Filed to exist");

    let mut bf_inter = BFSimpleInterpreter::new(program_source);

    let start_time = time::Instant::now();

    bf_inter.execute();

    let end_time = time::Instant::now();

    let program_run_time = end_time.duration_since(start_time).as_secs_f64();

    println!("Instructions Evaluated {:?}", bf_inter.instruction_count());
    println!("Time Elapsed {:?}", program_run_time);
}
