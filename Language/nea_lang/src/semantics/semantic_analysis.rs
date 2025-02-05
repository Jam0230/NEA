use crate::parser::parser::{Decl, Expr, Stmt};
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
        return match self {
            SymbolTypes::Int => String::from("Int"),
            SymbolTypes::Float => String::from("Float"),
            SymbolTypes::Str => String::from("Str"),
            SymbolTypes::Bool => String::from("Bool"),
            SymbolTypes::Char => String::from("Char"),
        };
    }

    fn from_string(type_s: String) -> SymbolTypes {
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
    fn new(symbols: Vec<Symbol>, level: ScopeLevels) -> Self {
        Self { symbols, level }
    }

    fn search_for_symbol(self, symbol: String) -> bool {
        return if self.symbols.iter().filter(|x| x.name == symbol).count() == 0 {
            false
        } else {
            true
        };
    }
}

fn search_scope_stack(symbol: String, scope_stack: Vec<SymbolTable>) -> Option<SymbolTypes> {
    let mut result: Option<SymbolTypes> = None;

    for symbol_table in scope_stack {
        let symbol = symbol_table.symbols.into_iter().find(|x| x.name == symbol);
        match symbol {
            Some(symbol) => result = Some(symbol.s_type),
            None => {}
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
            let _type =
                search_scope_stack(current_expr.val.clone().unwrap(), symbol_tables.clone());

            match _type {
                Some(t) => return Ok(t),
                None => {
                    //TODO: You know what im going to say here :3
                    return Err(format!(
                        "No symbol '{}' in current scope",
                        current_expr.val.unwrap()
                    ));
                }
            }
        }
        "Int" | "Float" | "Str" | "Bool" | "Char" => {
            return Ok(SymbolTypes::from_string(current_expr.expr_type));
        }
        "Eq" | "Neq" | "Lt" | "Gt" | "LtEq" | "GtEq" | "Add" | "Sub" | "Mul" | "Div" | "Mod"
        | "LogAnd" | "LogOr" => {
            let operation_table = operation_table::load_operation_table();

            let left = type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone());
            let right = type_check_expr(*current_expr.right.unwrap(), symbol_tables.clone());

            match (left.clone(), right.clone()) {
                (Ok(l), Ok(r)) => {
                    let l_string = l.stringify();
                    let r_string = r.stringify();

                    let result_type = operation_table.get(&(
                        current_expr.expr_type.as_str(),
                        l_string.as_str(),
                        r_string.as_str(),
                    ));

                    match result_type {
                        Some(r) => return Ok(SymbolTypes::from_string(String::from(*r))),
                        None => {
                            return Err(format!(
                                "cannot {} a {} and a {}",
                                current_expr.expr_type.as_str(),
                                l_string,
                                r_string,
                            ))
                        }
                    }
                }
                _ => return Err(format!("{:?} - {:?}", left, right)),
            }
        }
        "LogNot" => {
            let val = type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone());

            if val.clone().unwrap() == SymbolTypes::Bool {
                return Ok(SymbolTypes::Bool);
            }
            return Err(format!("Cannot not a '{:?}'", val)); //TODO: Yet another temp error :3
        }
        "Group" => {
            return type_check_expr(*current_expr.left.unwrap(), symbol_tables.clone());
        }
        _ => {
            //TODO: Turn into error once error handling complete
            return Err(String::from("Type_check Error"));
        }
    }
}

fn enter_local_scope(
    current_stmt: Stmt,
    mut scope_stack: Vec<SymbolTable>,
    mut errors: i32,
) -> i32 {
    println!("entering scope");

    scope_stack.push(SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Local,
    });

    errors = traverse_ast(current_stmt, scope_stack, errors);
    return errors;
}

pub fn traverse_ast(
    mut current_stmt: Stmt,
    mut scope_stack: Vec<SymbolTable>,
    mut errors: i32,
) -> i32 {
    loop {
        println!("\n{}", current_stmt.stmt_type);

        match current_stmt.stmt_type.as_str() {
            "IfStmt" => {
                // check condition
                let expr_type =
                    type_check_expr(current_stmt.expr.clone().unwrap(), scope_stack.clone());
                match expr_type {
                    Ok(t) => {
                        if t != SymbolTypes::Bool {
                            //TODO: Make this error when error handling implemented :D
                            println!("Expected type 'Bool', found type '{}'", t.stringify());
                            errors += 1;

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                        println!("Yay type is correct");
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
                                //TODO: Make this error when error handling implemented :D
                                println!("Expected type 'Bool', found type '{}'", t.stringify());
                                errors += 1;

                                if current_stmt.next.is_some() {
                                    current_stmt = *current_stmt.next.unwrap();
                                } else {
                                    return errors;
                                }
                                continue;
                            }
                            println!("Yay type is correct");
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
                            //TODO: Make this error when error handling implemented :D
                            println!("Expected type 'Bool', found type '{}'", t.stringify());
                            errors += 1;

                            if current_stmt.next.is_some() {
                                current_stmt = *current_stmt.next.unwrap();
                            } else {
                                return errors;
                            }
                            continue;
                        }
                        println!("Yay type is correct");
                    }
                    Err(e) => {
                        println!("{}", e);
                        errors += 1
                    }
                }
                // enter body (if exists)You need to complete the student progress document attached to this hoomework prior to your meeting.
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }
            }
            "ForStmt" => {
                // This one accually sucks so :P
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
                        .clone()
                        .unwrap()
                        .id
                        .unwrap(),
                    s_type: match current_stmt
                        .clone()
                        .stmt_1
                        .unwrap()
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
                    Err(_) => {
                        println!("D: (very descriptive");
                        errors += 1;
                    } //TODO: temp error 3: electric boogy
                }

                // Check body
                if current_stmt.body.is_some() {
                    errors =
                        enter_local_scope(*current_stmt.body.unwrap(), scope_stack.clone(), errors);
                }
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
                    ); //TODO: temperary error, implement once error handling complete
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
                // TODO: This code sucks make it better please :3
                match type_check_expr(
                    current_stmt.decl_node.clone().unwrap().value.unwrap(),
                    scope_stack.clone(),
                ) {
                    Ok(t) => {
                        let symbol_type = search_scope_stack(
                            current_stmt.decl_node.unwrap().id.unwrap(),
                            scope_stack.clone(),
                        );

                        if symbol_type.is_some() {
                            if symbol_type.clone().unwrap() != t {
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
                            println!("Yay type is correct");
                        }
                        println!("")
                    }
                    Err(_) => {
                        println!("D:");
                        errors += 1;
                    } //TODO: temp error 2: electric boogaloo
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
                // TODO: This code sucks still make it better please :3
                match type_check_expr(
                    current_stmt.decl_node.clone().unwrap().value.unwrap(),
                    scope_stack.clone(),
                ) {
                    Ok(t) => {
                        let symbol_type = search_scope_stack(
                            current_stmt.decl_node.clone().unwrap().id.unwrap(),
                            scope_stack.clone(),
                        );

                        if symbol_type.is_some() {
                            if symbol_type.clone().unwrap() != t {
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
                            println!("Yay type is correct");
                        }
                    }
                    Err(_) => {
                        println!("D:");
                        errors += 1
                    } //TODO: IDK maybe  make this an error :3
                }
            }
            _ => {} // TODO: Count as error, once error handling
        }

        // println!("{:?}", scope_stack);

        if current_stmt.next.is_some() {
            current_stmt = *current_stmt.next.unwrap();
        } else {
            return errors;
        }
    }
}

pub fn semantic_analyser(ast: Stmt) -> i32 {
    let scope_stack: Vec<SymbolTable> = vec![SymbolTable {
        symbols: Vec::new(),
        level: ScopeLevels::Global,
    }];

    let errors = traverse_ast(ast, scope_stack, 0);
    println!("Num Errors: {}", errors);
    errors
}

// i dont like this code :/
