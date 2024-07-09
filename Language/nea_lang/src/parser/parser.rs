use crate::{parser::parse_table::load_parse_table, scanner::finite_automata::Token};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
struct Node {
    _type: String,
    content: Option<String>,
    children: Vec<Node>,
}

// types of nodes (miniaturised language)
//
// stmt (statement):
//     type = If, elif, else, while, decl, assign
//     decl = decleration node if decl/assign type
//     expr = expression linked to stmt
//     body = stmt list executed if stmt passes
//     next_if = next statement in if_elif_else stmt
//     next = next statement in list
// decl (decleration):
//     type = decl, assign
//     Id = identifier used in decl/assign
//     var_Type = type of variable (literal)
//     value = value of variable (expression node)
//     ass_type = assign operator used (for assign node)
// expr (expression):
//     type = all operators and literal types ( +, -, ect)
//     right = expr on right (only operators)
//     left = expr on left (only operators and unary )
//     value = value of literals  ( only literals )
// var_type:
//     type = all literal types
//
// other posible in future:
// param (parameter list)
// and more parts for current nodes (case statement)
//
// i dont want to code this ;-;

impl Node {
    fn new(_type: String, content: Option<String>, children: Vec<Node>) -> Node {
        Node {
            _type,
            content,
            children,
        }
    }

    fn compress(&self) -> Option<Node> {
        if self._type == String::from("Terminal") {
            return Some(self.clone());
        }

        if self.children.len() == 0 {
            return None;
        }

        let mut new_children: Vec<Node> = Vec::new();

        for child in &self.children {
            match child.compress() {
                Some(compressed_child) => new_children.push(compressed_child),
                None => {}
            }
        }

        if !["<P>", "<SS>", "<S>"].contains(&self._type.as_str()) {
            if new_children.len() == 1 {
                return Some(new_children.iter().next().unwrap().clone());
            } else {
                let new_node = Node::new(self._type.clone(), self.content.clone(), new_children);
                return Some(new_node);
            }
        }

        let new_node = Node::new(self._type.clone(), self.content.clone(), new_children);
        return Some(new_node);
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.content.is_some() {
            write!(f, "{}: {}", self._type, self.content.clone().unwrap())
        } else {
            write!(f, "{}", self._type)
        }
    }
}

fn print_tree(node: &Node, level: u32) {
    println!("{}{}", (0..level).map(|_| " |").collect::<String>(), node);

    for child in node.children.clone().iter().rev() {
        print_tree(&child, level + 1);
    }
}

pub fn parse(tokens: &mut Vec<Token>) {
    let hash = load_parse_table();

    // the two stacks for table driven parsing
    let mut token_index = 0;
    let mut stack = vec!["$", "<P>"];

    let mut node_stack: Vec<Node> = Vec::new();

    while !stack.is_empty() {
        let temp = format!("{:?}", stack.iter().rev().collect::<Vec<&&str>>());
        let (next_s, next_i) = (stack.pop().unwrap(), tokens[token_index].clone());
        println!("\n{} ||| {}", next_i, temp);

        for node in node_stack.clone() {
            println!("{}", node);
        }

        if next_s.chars().next().unwrap() == '|' {
            let split_token = next_s[1..next_s.len() - 1]
                .split(",")
                .collect::<Vec<&str>>();
            let num_to_collect = split_token[0].parse::<u32>().unwrap();
            let collect_type = split_token[1];

            let mut collected_nodes: Vec<Node> = Vec::new();
            for _ in 0..num_to_collect {
                collected_nodes.push(node_stack.pop().unwrap());
            }
            let new_node = Node::new(String::from(collect_type), None, collected_nodes);

            node_stack.push(new_node);

            println!("{}-{}", num_to_collect, collect_type);

            continue;
        }

        // if both stack and input stream have the same terminal on top
        if next_s == next_i.contents.as_str() || next_s == format!("[{}]", next_i._type).as_str() {
            token_index += 1;
            if next_s != "$" {
                let new_node =
                    Node::new(String::from("Terminal"), Some(next_i.contents), Vec::new());
                node_stack.push(new_node);
            }
            continue;
        }

        // check type of token for rule
        match hash.get(&(next_s, format!("[{}]", next_i._type).as_str())) {
            Some(symbols) => {
                for symbol in symbols.iter().rev() {
                    stack.push(symbol);
                }
                continue;
            }
            None => {
                // check contents of token for rule
                match hash.get(&(next_s, next_i.contents.as_str())) {
                    Some(symbols) => {
                        for symbol in symbols.iter().rev() {
                            stack.push(symbol);
                        }
                        continue;
                    }
                    None => {
                        println!("Oh no >:(");
                        break;
                    }
                }
            }
        }
    }
    print_tree(&node_stack[0].compress().unwrap(), 0);
}
