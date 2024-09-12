use crate::parser::parser::{Decl, Expr, Stmt};

#[derive(Debug, Clone, PartialEq)]
enum SymbolTypes {
    Int,
    Float,
    Str,
    Bool,
    Char,
}

#[derive(Debug, Clone)]
enum ScopeLevels {
    Global,     // Global variables
    Parameters, // Parameters in the declerations of functions (currently unused)
    Local,      // everything else ( variables defined in the middle of expression bodies)
}

#[derive(Debug, Clone, PartialEq)]
struct Symbol {
    name: String,
    s_type: SymbolTypes,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
    level: ScopeLevels,
}

impl SymbolTable {
    fn search_for_symbol(self, symbol: String) -> bool {
        return if self.symbols.iter().filter(|x| x.name == symbol).count() == 0 {
            false
        } else {
            true
        };
    }
}

fn enter_local_scope(current_stmt: Stmt, mut scope_stack: Vec<SymbolTable>) {
    println!("entering scope :3");

    scope_stack.push(SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Local,
    });

    traverse_ast(current_stmt, scope_stack);
}

pub fn traverse_ast(mut current_stmt: Stmt, mut scope_stack: Vec<SymbolTable>) {
    loop {
        println!("{}", current_stmt.stmt_type);

        match current_stmt.stmt_type.as_str() {
            "IfStmt" => {
                // check condition

                // enter body of if stmt (if exists)
                if current_stmt.body.is_some() {
                    enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone());
                }
                // enter elif stmt branch (if exists)
                if current_stmt.elif_stmt.is_some() {
                    traverse_ast(*current_stmt.elif_stmt.unwrap(), scope_stack.clone());
                }
                // enter else body (if exists)
                if current_stmt.else_stmt.is_some() {
                    traverse_ast(*current_stmt.else_stmt.unwrap(), scope_stack.clone());
                }
            }
            "ElifStmt" | "ElseStmt" => {
                // check condition (elif)
                // enter body (if exists)
                if current_stmt.body.is_some() {
                    enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone());
                }
            }
            "WhileStmt" => {
                // check condition
                // enter body (if exists)
                if current_stmt.body.is_some() {
                    enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone());
                }
            }
            "DeclStmt" => {
                // check if type of ID matches expression
                // stop if variable exists in scope
                if scope_stack
                    .last()
                    .unwrap()
                    .clone()
                    .search_for_symbol(current_stmt.decl_node.clone().unwrap().id.unwrap())
                {
                    println!(
                        "Identifier '{}' already exists in current scope!",
                        current_stmt.decl_node.clone().unwrap().id.unwrap()
                    ); //TODO: temperary error, implement once error handling complete

                    if current_stmt.next.is_some() {
                        current_stmt = *current_stmt.next.unwrap();
                        continue;
                    } else {
                        return;
                    }
                }
                // add symbol of symbol table
                scope_stack.last_mut().unwrap().symbols.push(Symbol {
                    name: current_stmt.decl_node.clone().unwrap().id.unwrap(),
                    s_type: match current_stmt.decl_node.unwrap().var_type.unwrap().as_str() {
                        "int" => SymbolTypes::Int,
                        "float" => SymbolTypes::Float,
                        "str" => SymbolTypes::Str,
                        "bool" => SymbolTypes::Bool,
                        "char" => SymbolTypes::Char,
                        _ => SymbolTypes::Int, // shouldnt happen but rust insists
                    },
                });
            }
            "AssignStmt" => {
                // stop if variable doesnt exist in scope

                let mut in_scope = false;
                for symbol_table in scope_stack.iter().rev() {
                    if symbol_table
                        .clone()
                        .search_for_symbol(current_stmt.decl_node.clone().unwrap().id.unwrap())
                    {
                        in_scope = true;
                        break;
                    }
                }
                if !(in_scope) {
                    println!(
                        "Identifer '{}' does not exist in current scope!",
                        current_stmt.decl_node.clone().unwrap().id.unwrap()
                    );
                }

                // check if type of ID matches expression
            }
            _ => {} // TODO: Count as error, once error handling
        }

        // println!("{:?}", scope_stack);

        if current_stmt.next.is_some() {
            current_stmt = *current_stmt.next.unwrap();
        } else {
            return;
        }
    }
}

pub fn semantic_analyser(ast: Stmt) {
    let scope_stack: Vec<SymbolTable> = vec![SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Global,
    }];

    traverse_ast(ast, scope_stack);
}

// i dont like this code :/
