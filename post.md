---
layout: post
title: Writing an optimizing compiler for BF - Interpreter
date: 2025-04-08
categories:
  - Personal Projects
tags:
  - Rust
  - BF
---

I have had some time after my defense, to look at topics that I wanted to tackle but simply never had the time to do so. I have wanted to make a programming language since I learned how to program, but jumping from a linear algebra junkie to language design is a bit of a large step. I have been reading "Crafting Interpreters" by Robert Nystrom and "Writing an Interpreter in Go" by Thorsten Ball, which are great books, however made me realize to fully understand each state of the process, I needed a simpler base to start on this journey.

This is the start of a (hopefully) series of blog posts that will end with making a virtual machine and optimizing compiler for the language [BrainF***](https://en.wikipedia.org/wiki/Brainfuck). It is a simple language with only 8 commands defined as single characters. This is a somewhat popular language inside of the CS community as the name is fun, and it is a wildly impractical language that is still Turing complete. I will not define for you this language as much better write-ups are available [elsewhere](https://esolangs.org/wiki/Brainfuck).

However, in this post I will describe building an interpreter for BF in Rust. This implementation is heavily inspired by Thorsten Ball's 
interpreter [implimentation](https://thorstenball.com/blog/2017/01/04/a-virtual-brainfuck-machine-in-go/). 


```Rust
pub trait BFExecuter {
    fn execute(&mut self) -> ();
    fn read_char(&mut self) -> ();
    fn write_char(&mut self) -> ();
    fn instruction_count(&mut self) -> usize;
}
```

```Rust
pub struct ProgramState {
    pub ip: usize,
    pub dp: usize,
    pub memory: [i32; 30000],
}
```

```Rust
pub struct BFSimpleInterpreter {
    program: Vec<char>,
    machine: ProgramState,
    stdin: Stdin,
    stdout: Stdout,
    inst_evaluated: usize,
}
```


```Rust
impl BFExecuter for BFSimpleInterpreter {
    fn execute(&mut self) -> () {
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
                    self.machine.dp += 1;
                }
                '<' => {
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

    fn read_char(&mut self) -> () {
        let mut byte = [0_u8];
        self.stdin.lock().read_exact(&mut byte).expect("");
        self.machine.memory[self.machine.dp] = byte[0] as i32;
    }

    fn write_char(&mut self) -> () {
        let mut byte = [0_u8];
        byte[0] = self.machine.memory[self.machine.dp] as u8;
        self.stdout.write(&byte).expect("");
    }

    fn instruction_count(&mut self) -> usize {
        self.inst_evaluated
    }
}
```


```Rust
fn main() {
    // generate the bf program path
    let program_path_str = env::args().collect::<Vec<String>>()[1].clone();
    let program_path= Path::new(&program_path_str);

    let program_source = fs::read_to_string(program_path).expect("File to exist");

    let mut bf_inter = BFSimpleInterpreter::new(program_source);

    let start_time = time::Instant::now();

    bf_inter.execute();

    let end_time = time::Instant::now();

    let program_run_time = end_time.duration_since(start_time).as_secs_f64();

        println!("Instructions {:?}", bf_inter.instruction_count());
        println!("Time Elapsed {:?} sec.", program_run_time);
}
```


We can now try to run a BF example, hello world.

```bash
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```

```bash
.\bf .\hello_world.bf
Hello, World!

Instructions 974
Time Elapsed 0.0001163 sec.
```


However, when we try to compute an ascii version of the Mandelbrot set via BF, we see that it is quite slow (>30 sec)
on a modern desktop CPU. And we are some fairly important things missing, such as how do we validate we have a valid 
program, and, you know, optimization. We are operating at ~350 million instructions per second, which on its own is 
not bad at all, but these instructions are very simple and encode very little.


```bash
.\bf .\mandelbrot.bf

......

Instructions 10904007066
Time Elapsed 30.4994101 sec.
```

The next post will cover making an instruction set and a virtual machine to run it on. All code for this post can be 
found in the following [github repo](ds).