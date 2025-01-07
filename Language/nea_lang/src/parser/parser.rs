use crate::{parser::parse_table::load_parse_table, scanner::scanner::Token};

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
//     value = value of literals  ( only literals, one for each type )
// var_type:
//     type = all literal types
//
// other posible in future:
// param (parameter list)
// and more parts for current nodes (case statement)

#[derive(Debug, Clone)]
pub struct Stmt {
    pub stmt_type: String,            // type of statement ( if, else, while, ..)
    pub decl_node: Option<Decl>,      // ast decleration node if decleration statement
    pub expr: Option<Expr>,           // expression linked to statement ( conditions )
    pub body: Option<Box<Stmt>>,      // stmt branch linked to stmt
    pub elif_stmt: Option<Box<Stmt>>, // elif statement branch connected to if stmt
    pub else_stmt: Option<Box<Stmt>>, // else statement connected to if stmt
    pub next: Option<Box<Stmt>>,      // next statement in branch
}

#[derive(Debug, Clone)]
pub struct Decl {
    pub decl_type: String,  // type of declerations (assignment, decleration, ..)
    pub id: Option<String>, // identifier used in decleration
    pub var_type: Option<String>, // variable type used in decleration
    pub value: Option<Expr>, // expression branch linked to decl
    pub ass_type: Option<String>, // assignment type used for assignment
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub expr_type: String,        // expression type ( operator or literal )
    pub left: Option<Box<Expr>>,  // expression on left of operators
    pub right: Option<Box<Expr>>, // expression of right of operator
    pub val: Option<String>,      // value of literal or identifier name
}

#[derive(Debug, Clone)]
enum AstItem {
    // used in the ast stack to allow multiple types in one vector
    Stmt(Stmt),
    Decl(Decl),
    Expr(Expr),
    Terminal(String),
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Stmt, String> {
    let hash = load_parse_table();

    // index of token being parsed
    let mut token_index = 0;

    // the two stacks for table driven parsing
    let mut stack = vec!["$", "<SS>"];
    let mut ast_stack: Vec<AstItem> = Vec::new();

    while !stack.is_empty() {
        let (next_s, next_i) = (stack.pop().unwrap(), tokens[token_index].clone());

        if next_s.chars().next().unwrap() == '|' && next_s.chars().nth(1).unwrap() != '|' {
            // collection node found
            let parts = next_s[1..next_s.len() - 1]
                .split(',')
                .collect::<Vec<&str>>();

            let (node_type, node_type_type) = (
                parts[0].split('-').nth(0).unwrap(),
                parts[0].split('-').nth(1).unwrap(),
            );

            let num_collected = parts[1].parse::<usize>().unwrap();

            let mut collected_values: Vec<AstItem> = Vec::new(); // collecting the AST stack items
                                                                 // used by the new node
            for _ in 0..num_collected {
                collected_values.push(ast_stack.pop().unwrap());
            }

            let mut parameters: Vec<Option<AstItem>> = Vec::new();

            for char in parts[2].chars() {
                // setting parameters for the node creation functions
                if char == '_' {
                    parameters.push(None);
                    continue;
                }

                let index = char.to_string().parse::<usize>().unwrap();
                parameters.push(Some(collected_values[index - 1].clone()));
            }

            println!("{:?}", parameters);

            match node_type {
                // node creationg functions
                "Stmt" => {
                    let stmt = Stmt {
                        stmt_type: node_type_type.to_string(),
                        decl_node: match parameters[0].clone() {
                            Some(AstItem::Decl(decl)) => Some(decl),
                            _ => None,
                        },
                        expr: match parameters[1].clone() {
                            Some(AstItem::Expr(expr)) => Some(expr),
                            _ => None,
                        },
                        body: match parameters[2].clone() {
                            Some(AstItem::Stmt(stmt)) => {
                                if stmt.stmt_type == "None" {
                                    None
                                } else {
                                    Some(Box::new(stmt))
                                }
                            }
                            _ => None,
                        },
                        elif_stmt: match parameters[3].clone() {
                            Some(AstItem::Stmt(stmt)) => {
                                if stmt.stmt_type == "None" {
                                    None
                                } else {
                                    Some(Box::new(stmt))
                                }
                            }
                            _ => None,
                        },
                        else_stmt: match parameters[4].clone() {
                            Some(AstItem::Stmt(stmt)) => {
                                if stmt.stmt_type == "None" {
                                    None
                                } else {
                                    Some(Box::new(stmt))
                                }
                            }
                            _ => None,
                        },
                        next: match parameters[5].clone() {
                            Some(AstItem::Stmt(stmt)) => {
                                if stmt.stmt_type == "None" {
                                    None
                                } else {
                                    Some(Box::new(stmt))
                                }
                            }
                            _ => None,
                        },
                    };
                    ast_stack.push(AstItem::Stmt(stmt));
                }
                "Decl" => {
                    let decl = Decl {
                        decl_type: node_type_type.to_string(),
                        id: match parameters[0].clone() {
                            Some(AstItem::Terminal(term)) => Some(term),
                            _ => None,
                        },
                        var_type: match parameters[1].clone() {
                            Some(AstItem::Terminal(term)) => Some(term),
                            _ => None,
                        },
                        value: match parameters[2].clone() {
                            Some(AstItem::Expr(expr)) => Some(expr),
                            _ => None,
                        },
                        ass_type: match parameters[3].clone() {
                            Some(AstItem::Terminal(term)) => Some(term),
                            _ => None,
                        },
                    };
                    ast_stack.push(AstItem::Decl(decl));
                }
                "Expr" => {
                    let expr = Expr {
                        expr_type: node_type_type.to_string(),
                        left: match parameters[0].clone() {
                            Some(AstItem::Expr(expr)) => Some(Box::new(expr)),
                            _ => None,
                        },
                        right: match parameters[1].clone() {
                            Some(AstItem::Expr(expr)) => Some(Box::new(expr)),
                            _ => None,
                        },
                        val: match parameters[2].clone() {
                            Some(AstItem::Terminal(term)) => Some(term),
                            _ => None,
                        },
                    };
                    ast_stack.push(AstItem::Expr(expr));
                }
                _ => {}
            }

            continue;
        }

        // if both stack and input stream have the same terminal on top
        if next_s == next_i.contents.as_str() || next_s == format!("[{}]", next_i._type).as_str() {
            token_index += 1;

            if next_s != "$" {
                ast_stack.push(AstItem::Terminal(next_i.contents));
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
                        // syntax error
                        // TODO: set up error handling :(
                        println!("Oh no >:(");
                        break;
                    }
                }
            }
        }
    }
    match ast_stack[0].clone() {
        AstItem::Stmt(stmt) => Ok(stmt),
        _ => Err(String::new()),
    }
}
