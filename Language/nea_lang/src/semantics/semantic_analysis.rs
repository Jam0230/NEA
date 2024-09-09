use crate::parser::parser::{Decl, Expr, Stmt};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Symbol {
    name: String,
    s_type: SymbolTypes,
}

#[derive(Debug, Clone)]
struct SymbolTable {
    symbols: Vec<Symbol>,
    level: ScopeLevels,
}

fn enter_local_scope(current_stmt: Stmt, mut scope_stack: Vec<SymbolTable>) {
    println!("entering scope :3");

    scope_stack.push(SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Local,
    });

    traverse_ast(current_stmt, scope_stack.clone());

    scope_stack.pop();
}

fn traverse_ast(current_stmt: Stmt, scope_stack: Vec<SymbolTable>) {
    println!("{}", current_stmt.stmt_type);
    println!("{:?}", scope_stack);

    let mut scope_stack = scope_stack;
    match current_stmt.stmt_type.as_str() {
        "IfStmt" => {
            //  check condition (;-;)

            // enter body of if stmt (if exists)
            if current_stmt.body.is_some() {
                enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone());
            }
            // enter stmt branch of if else stmt's (if exists)
            if current_stmt.elif_stmt.is_some() {
                traverse_ast(*current_stmt.elif_stmt.unwrap(), scope_stack.clone());
            }
            // enter body of else stmt (if exists)
            if current_stmt.else_stmt.is_some() {
                traverse_ast(*current_stmt.else_stmt.unwrap(), scope_stack.clone());
            }
        }
        "ElifStmt" | "ElseStmt" => {
            // check condition (if elif)

            // enter body of stmt (if exists)
            if current_stmt.body.is_some() {
                enter_local_scope(*current_stmt.body.unwrap(), scope_stack);
            }
        }
        "WhileStmt" => {
            // check condition (;-;)

            // enter body
            if current_stmt.body.is_some() {
                enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone());
            }
        }
        "AssignStmt" | "DeclStmt" => {}
        _ => {}
    }
}

pub fn semantic_analyser(ast: Stmt) {
    let scope_stack: Vec<SymbolTable> = vec![SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Global,
    }];

    traverse_ast(ast, scope_stack);
}
