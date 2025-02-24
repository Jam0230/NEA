use crate::parser::parser::{Expr, Stmt};
use crate::semantics::operation_table;

#[derive(Debug, Clone, PartialEq)]
enum SymbolTypes {
    Int,
    Float,
    Str,
    Bool,
    Char,
}

impl SymbolTypes {
    fn stringify(self) -> String {
        // Parses a type into a string
        match self {
            SymbolTypes::Int => String::from("Int"),
            SymbolTypes::Float => String::from("Float"),
            SymbolTypes::Str => String::from("Str"),
            SymbolTypes::Bool => String::from("Bool"),
            SymbolTypes::Char => String::from("Char"),
        }
    }

    fn from_string(type_s: String) -> SymbolTypes {
        // Parses a string into a type
        match type_s.as_str() {
            "Int" => SymbolTypes::Int,
            "Float" => SymbolTypes::Float,
            "Str" => SymbolTypes::Str,
            "Bool" => SymbolTypes::Bool,
            "Char" => SymbolTypes::Char,
            _ => SymbolTypes::Int,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Symbol {
    // Identifier
    name: String,
    // Type
    s_type: SymbolTypes,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    // Table of Identifiers in current scope
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    // returns true if searched symbol is in the table
    fn search_for_symbol(self, symbol: String) -> bool {
        self.symbols.iter().filter(|x| x.name == symbol).count() != 0
    }
}

fn search_scope_stack(symbol: String, scope_stack: Vec<SymbolTable>) -> Option<SymbolTypes> {
    // Searches all currently alive stacks for a symbol and returns its type
    let mut result: Option<SymbolTypes> = None;

    for symbol_table in scope_stack {
        let symbol = symbol_table.symbols.into_iter().find(|x| x.name == symbol);
        if let Some(symbol) = symbol {
            result = Some(symbol.s_type)
        }
    }

    result
}

fn type_check_expr(
    current_expr: Expr,
    symbol_tables: Vec<SymbolTable>,
) -> Result<SymbolTypes, String> {
    match current_expr.expr_type.as_str() {
        "Id" => {
            // Identifier

            // get type from symbol table
            let _type =
                search_scope_stack(current_expr.val.clone().unwrap(), symbol_tables.clone());

            match _type {
                Some(t) => Ok(t),
                None => Err(format!(
                    // return error if symbol not found
                    "No symbol '{}' in current scope",
                    current_expr.val.unwrap()
                )),
            }
        }
        "Int" | "Float" | "Str" | "Bool" | "Char" => {
            // Literal
            Ok(SymbolTypes::from_string(current_expr.expr_type))
        }
        "Eq" | "Neq" | "Lt" | "Gt" | "LtEq" | "GtEq" | "Add" | "Sub" | "Mul" | "Div" | "Mod"
        | "LogAnd" | "LogOr" => {
            // Binary Operation
            let operation_table = operation_table::load_operation_table();

            // type check both sides of the operation
            let left = type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone());
            let right = type_check_expr(*current_expr.right.unwrap(), symbol_tables.clone());

            match (left.clone(), right.clone()) {
                (Ok(l), Ok(r)) => {
                    // both sides return a type
                    let l_string = l.stringify();
                    let r_string = r.stringify();

                    let result_type = operation_table.get(&(
                        // check opearation table for the types used
                        current_expr.expr_type.as_str(),
                        l_string.as_str(),
                        r_string.as_str(),
                    ));

                    match result_type {
                        // return type from operation table if types are allowed for operation
                        Some(r) => Ok(SymbolTypes::from_string(String::from(*r))),
                        None => Err(format!(
                            // Combination of types not found in operation table
                            "Could not perform operation '{}' with types: \n\t{:?} - {:?}",
                            current_expr.expr_type.as_str(),
                            l_string,
                            r_string,
                        )),
                    }
                }
                _ => Err(format!(
                    // return error if a side didnt return a type
                    "Could not perform operation '{}' with types: \n\t{:?} - {:?}",
                    current_expr.expr_type, left, right
                )),
            }
        }
        "LogNot" => {
            // Unary Operation
            let val = type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone());

            if val.clone().unwrap() == SymbolTypes::Bool {
                // if type isnt a boolean return error
                return Ok(SymbolTypes::Bool);
            }
            Err(format!("Cannot not a '{:?}'", val))
        }
        // return type of contents of group
        "Group" => type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone()),
        _ => Err(format!(
            "Unknown Expression type {}",
            current_expr.expr_type
        )),
    }
}

fn enter_local_scope(current_stmt: Stmt, mut scope_stack: Vec<SymbolTable>, errors: i32) -> i32 {
    // adds a new symbol table to the scope stack
    scope_stack.push(SymbolTable {
        symbols: Vec::new(),
    });

    traverse_ast(current_stmt, scope_stack, errors)
}

fn traverse_ast(mut current_stmt: Stmt, mut scope_stack: Vec<SymbolTable>, mut errors: i32) -> i32 {
    loop {
        match current_stmt.stmt_type.as_str() {
            "IfStmt" => {
                // check condition
                let expr_type =
                    type_check_expr(current_stmt.expr.clone().unwrap(), scope_stack.clone());
                match expr_type {
                    Ok(t) => {
                        if t != SymbolTypes::Bool {
                            println!("Expected type 'Bool', found type '{}'", t.stringify());
                            errors += 1;

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1;
                    }
                }

                // enter body of if stmt (if exists)
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }
                // enter elif stmt branch (if exists)
                if current_stmt.stmt_1.is_some() {
                    errors =
                        traverse_ast(*current_stmt.stmt_1.unwrap(), scope_stack.clone(), errors);
                }
                // enter else body (if exists)
                if current_stmt.stmt_2.is_some() {
                    errors =
                        traverse_ast(*current_stmt.stmt_2.unwrap(), scope_stack.clone(), errors);
                }
            }
            "ElifStmt" | "ElseStmt" => {
                // check condition (elif)
                if current_stmt.expr.is_some() {
                    let expr_type =
                        type_check_expr(current_stmt.expr.clone().unwrap(), scope_stack.clone());
                    match expr_type {
                        Ok(t) => {
                            if t != SymbolTypes::Bool {
                                println!("Expected type 'Bool', found type '{}'", t.stringify());
                                errors += 1;

                                if current_stmt.next.is_some() {
                                    current_stmt = *current_stmt.next.unwrap();
                                } else {
                                    return errors;
                                }
                                continue;
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            errors += 1
                        }
                    }
                }
                // enter body (if exists)
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }
            }
            "WhileStmt" => {
                // check condition
                let expr_type =
                    type_check_expr(current_stmt.expr.clone().unwrap(), scope_stack.clone());
                match expr_type {
                    Ok(t) => {
                        if t != SymbolTypes::Bool {
                            println!("Expected type 'Bool', found type '{}'", t.stringify());
                            errors += 1;

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1
                    }
                }
                // enter body (if exists)
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }
            }
            "ForStmt" => {
                // Check decleration statement (stmt_1)
                errors = enter_local_scope(
                    *current_stmt.clone().stmt_1.unwrap(),
                    scope_stack.clone(),
                    errors,
                );

                // add new variable to scope_stack
                scope_stack.last_mut().unwrap().symbols.push(Symbol {
                    name: current_stmt
                        .clone()
                        .stmt_1
                        .unwrap()
                        .decl_node
                        .unwrap()
                        .id
                        .unwrap(),
                    s_type: match current_stmt
                        .clone()
                        .stmt_1
                        .unwrap()
                        .decl_node
                        .unwrap()
                        .var_type
                        .unwrap()
                        .as_str()
                    {
                        "int" => SymbolTypes::Int,
                        "float" => SymbolTypes::Float,
                        "str" => SymbolTypes::Str,
                        "bool" => SymbolTypes::Bool,
                        "char" => SymbolTypes::Char,
                        _ => SymbolTypes::Int, // shouldnt happen but rust insists
                    },
                });

                // Check repeated assignment
                errors = enter_local_scope(
                    *current_stmt.clone().stmt_2.unwrap(),
                    scope_stack.clone(),
                    errors,
                );

                // Check condition
                match type_check_expr(current_stmt.expr.unwrap(), scope_stack.clone()) {
                    Ok(t) => {
                        if t != SymbolTypes::Bool {
                            println!("Expected Bool but found '{:?}'", t);

                            errors += 1;
                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1;
                    }
                }

                // Check body
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }

                scope_stack.last_mut().unwrap().symbols.pop();
            }
            "DeclStmt" => {
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
                    );
                    errors += 1;

                    if current_stmt.next.is_some() {
                        current_stmt = *current_stmt.next.unwrap();
                        continue;
                    } else {
                        return errors;
                    }
                }
                // add symbol of symbol table
                scope_stack.last_mut().unwrap().symbols.push(Symbol {
                    name: current_stmt.decl_node.clone().unwrap().id.unwrap(),
                    s_type: match current_stmt
                        .decl_node
                        .clone()
                        .unwrap()
                        .var_type
                        .unwrap()
                        .as_str()
                    {
                        "int" => SymbolTypes::Int,
                        "float" => SymbolTypes::Float,
                        "str" => SymbolTypes::Str,
                        "bool" => SymbolTypes::Bool,
                        "char" => SymbolTypes::Char,
                        _ => SymbolTypes::Int, // shouldnt happen but rust insists
                    },
                });
                // check if type of ID matches expression
                match type_check_expr(
                    current_stmt.decl_node.clone().unwrap().value.unwrap(),
                    scope_stack.clone(),
                ) {
                    Ok(t) => {
                        let symbol_type = search_scope_stack(
                            current_stmt.decl_node.unwrap().id.unwrap(),
                            scope_stack.clone(),
                        );

                        if symbol_type.is_some() && symbol_type.clone().unwrap() != t {
                            println!(
                                "Expected type '{}', found type '{}'",
                                symbol_type.unwrap().stringify(),
                                t.stringify()
                            );
                            errors += 1;

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1;
                    }
                }
            }
            "AssignStmt" => {
                // stop if variable doesnt exist in scop:we

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
                    errors += 1;
                }
                // check if type of ID matches expression
                match type_check_expr(
                    current_stmt.decl_node.clone().unwrap().value.unwrap(),
                    scope_stack.clone(),
                ) {
                    Ok(t) => {
                        let symbol_type = search_scope_stack(
                            current_stmt.decl_node.clone().unwrap().id.unwrap(),
                            scope_stack.clone(),
                        );

                        if symbol_type.is_some() && symbol_type.clone().unwrap() != t {
                            println!(
                                "Expected type '{}', found type '{}'",
                                symbol_type.unwrap().stringify(),
                                t.stringify()
                            );

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1
                    }
                }
            }
            _ => {}
        }

        if current_stmt.next.is_some() {
            current_stmt = *current_stmt.next.unwrap();
        } else {
            return errors;
        }
    }
}

pub fn semantic_analyser(ast: Stmt) -> i32 {
    // create a new scope_stack
    let scope_stack: Vec<SymbolTable> = vec![SymbolTable {
        symbols: Vec::new(),
    }];

    // semanticly analyse
    traverse_ast(ast, scope_stack, 0)
}
