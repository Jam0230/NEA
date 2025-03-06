use std::env;
use utils::file_input;

mod codegen;
mod parser;
mod scanner;
mod semantics;
mod utils;

fn print_help() {
    //prints the help information
    println!(
        "usage: tempname [OPTIONS] [FILE]

Options:
    -h / --help => displays help information
"
    );
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("No arguments given!\n try 'tempname --help' for more information");
        return;
    }

    for arg in args[1..].iter() {
        match arg.as_str() {
            "-h" | "--help" => {
                // print out help info and dont do anything else
                print_help();
                return;
            }
            _ => {
                if arg.chars().next() != Some('-') {
                    // if unknown arg and not last arg
                    if &args[args.len() - 1] != arg {
                        println!(
                            "Unknown argument '{}'!\n try 'tempname --help' for more information",
                            arg
                        );
                        return;
                    }

                    // file name
                    match file_input::read_file(arg) {
                        Ok(file_contents) => {
                            let mut tokens =
                                scanner::scanner::lexical_analyse(file_contents).expect("");

                            let _ast = parser::parser::parse(&mut tokens);

                            match _ast {
                                Ok(ast) => {
                                    // println!("{:#?}", ast);
                                    let semantic_errs =
                                        semantics::semantic_analysis::semantic_analyser(
                                            ast.clone(),
                                        );
                                    if semantic_errs != 0 {
                                        println!(
                                            "\n\nOh no, errors found :O\nthat means i have to quit :("
                                        );
                                        return;
                                    }
                                    let assembly = codegen::codegen::generate_assembly(ast);

                                    assembly.generate_assembly_file(arg.replace("txt", "asm"));
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }
                } else {
                    // unknown argument
                    println!(
                        "Unknown argument '{}'!\n try 'tempname --help' for more information",
                        arg
                    );
                }
            }
        }
    }
}
