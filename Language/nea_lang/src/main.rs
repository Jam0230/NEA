use std::env;
use utils::file_input;

use scanner::finite_automata;

mod parser;
mod scanner;
mod utils;

fn print_error(error: &str) {
    println!("{}\n try 'tempname --help' for more information", error);
}

fn print_help() {
    //prints the help information
    //TODO: Update this when adding new option for arguments
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
        print_error("No arguments given!");
        return;
    }

    for arg in args[1..].iter() {
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            _ => {
                if arg.chars().next() != Some('-') {
                    // file name
                    if &args[args.len() - 1] != arg {
                        print_error(format!("Unkown argument '{}'!", arg).as_str());
                        return;
                    }

                    match file_input::read_file(arg) {
                        Ok(file_contents) => {
                            let mut tokens =
                                finite_automata::lexical_analyse(file_contents).expect("");

                            parser::parser::parse(&mut tokens);
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }
                } else {
                    // unknown argument
                    print_error(format!("Unkown argument '{}'!", arg).as_str());
                }
            }
        }
    }

    // parser::parse_table::generate_parse_table();
}
