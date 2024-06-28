use crate::{parser::parse_table::load_parse_table, scanner::finite_automata::Token};
use std::thread;
use std::time::Duration;

fn get_first<T>(vec: &mut Vec<T>) -> Option<&T> {
    vec.first()
}

pub fn parse(tokens: &mut Vec<Token>) {
    let hash = load_parse_table();

    // the two stacks for table driven parcing
    let mut input_stream: Vec<&Token> = tokens.iter().rev().collect();
    let mut stack = vec!["$", "<P>"];

    while !stack.is_empty() {
        // thread::sleep(Duration::from_millis(500));
        let temp = format!("{:?} | ", stack);
        let (next_s, next_i) = (stack.pop().unwrap(), input_stream.pop().unwrap());
        println!("\n{}{}", temp, next_i);

        // if both stack and input stream have the same terminal on top
        if next_s == next_i.contents.as_str() || next_s == format!("[{}]", next_i._type).as_str() {
            continue;
        }

        // check type of token for rule
        match hash.get(&(next_s, format!("[{}]", next_i._type).as_str())) {
            Some(symbols) => {
                for symbol in symbols.iter().rev() {
                    stack.push(symbol);
                }
                input_stream.push(next_i);
                continue;
            }
            None => {}
        }

        // check contents of token for rule
        match hash.get(&(next_s, next_i.contents.as_str())) {
            Some(symbols) => {
                for symbol in symbols.iter().rev() {
                    stack.push(symbol);
                }
                input_stream.push(next_i);
                continue;
            }
            None => {
                println!("Oh no >:(");
                break;
            }
        }
    }
}
