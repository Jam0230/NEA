use crate::parser::parser::{Decl, Expr, Stmt};

struct Assembly {
    text_string: String,
    data_string: String,
    stack_offset: i32,
}

impl Assembly {
    fn new(text_string: String, data_string: String) -> Self {
        Self {
            text_string,
            data_string,
            stack_offset: 0,
        }
    }
}

/*
## Expressions ##

record movement in tree as binary number where 0 = left 1 = right

Explore mode :
Move through recorded movement (if a move is not possible left shift one and go to generate mode )
then repeat moving left until in a "dead end"
once in a "dead end" go to generate mode

generate mode :
generate current node
add one to move record if last move was left and go to explore mode
left shift 1 move record if last move was right and go to generate mode

*/

fn expression_node_gen(
    node: Expr,
    mut regs_used: Vec<String>,
) -> (
    String,      /*Node str*/
    String,      /*Data Str*/
    Vec<String>, /*Used Regs*/
) {
    let mut node_str = String::new();
    let mut data_str = String::new();

    let usable_regs_int = ["rax", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"];

    match node.expr_type.as_str() {
        // ## Literals ##
        "Int" | "Bool" | "Char" => {
            let mut register = String::new();
            for reg in usable_regs_int {
                if !regs_used.contains(&String::from(reg)) {
                    register = String::from(reg);
                    regs_used.push(String::from(reg));
                    break;
                }
            }

            if register == String::new() {
                print!("Expression is too chunky :O\n Please make it shorter for me :3");
                return (String::new(), String::new(), regs_used);
            }

            match node.expr_type.as_str() {
                "Int" => {
                    if ['x', 'i'].contains(&register.chars().last().unwrap()) {
                        register = register.replace("r", "e");
                    } else {
                        register = format!("{}d", register);
                    }
                }
                _ => {
                    if register.chars().last().unwrap() == 'x' {
                        register = format!("{}l", register.chars().nth(1).unwrap());
                    } else if register.chars().last().unwrap() == 'i' {
                        register = format!("{}l", register.replace("r", ""));
                    } else {
                        register += "b";
                    }
                }
            }

            node_str = format!("mov {}, {}", register, node.val.unwrap());
        }
        // ## Operations ##
        "Add" | "Sub" | "Mul" => {
            if let Some('r') = regs_used.last().unwrap().chars().nth(0) {
                // if integer operations
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let mut opcode = String::new();
                match node.expr_type.as_str() {
                    "Add" | "Sub" => {
                        opcode = node.expr_type.to_lowercase();
                    }
                    _ => {
                        opcode = String::from("imul");
                    }
                }

                node_str = format!("{} {}, {}", opcode, operand_1, operand_2);
                regs_used.push(operand_1);
            }
        }
        "Div" => {
            if let Some('r') = regs_used.last().unwrap().chars().nth(0) {
                // if integer Operations
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let mut register_1 = String::new();
                let mut register_2 = String::new();
                let mut done_1 = false;
                for i in 0..=15 {
                    if !regs_used.contains(&format!("xmm{}", i)) {
                        if !done_1 {
                            register_1 = format!("xmm{}", i);
                            done_1 = true;
                            continue;
                        } else {
                            register_2 = format!("xmm{}", i);
                            break;
                        }
                    }
                }

                node_str = format!(
                    "cvtsi2sd {reg_1}, {op_1}\ncvtsi2sd {reg_2}, {op_2}\ndivsd {reg_1}, {reg_2}",
                    op_1 = operand_1,
                    op_2 = operand_2,
                    reg_1 = register_1,
                    reg_2 = register_2
                );
                regs_used.push(register_1);
            }
        }
        _ => {
            println!("{}", node.expr_type);
        }
    }

    (node_str, data_str, regs_used)
}

fn rpngen(expression: Expr) -> (String, String) {
    let mut moves = vec![0];
    let mut skipleft = false;
    let mut expression_str = String::new();
    let mut data_str = String::new();

    let mut regs_used: Vec<String> = Vec::new();

    loop {
        let mut current_node = expression.clone();
        for _move in moves.clone() {
            if _move == 0 && current_node.left.is_some() {
                current_node = *current_node.left.unwrap();
            } else if _move == 1 && current_node.right.is_some() {
                current_node = *current_node.right.unwrap();
            } else {
                moves.pop();
                skipleft = true;
                break;
            }
        }
        while current_node.left.is_some() && !skipleft {
            current_node = *current_node.left.unwrap();
            moves.push(0);
        }

        let (node_str, node_data_str, regs_used_temp) =
            expression_node_gen(current_node, regs_used);
        regs_used = regs_used_temp;
        expression_str = format!("{}\n{}", expression_str, node_str);
        data_str = format!("{}\n{}", data_str, node_data_str);

        if moves.len() == 0 {
            break;
        }

        if moves.last().unwrap() == &0 {
            moves.pop();
            moves.push(1);
            skipleft = false;
        } else {
            moves.pop();
            skipleft = true;
        }
    }
    (expression_str, data_str)
}

/*
 ## Generating assembly ##

recursivly go through the ast adding the generated assembly to the correct string in the assembly struct
 - Normal code goes in "text_string"
 - Data for floating point and string constants in "data_string"
 :qa
  */
fn recursive_gen(current_stmt: Stmt, mut assembly: Assembly) -> Assembly {
    match current_stmt.stmt_type.as_str() {
        "DeclStmt" => {
            let (expression_str, expr_data_str) =
                rpngen(current_stmt.clone().decl_node.unwrap().value.unwrap());

            let mut decl_str = String::new();

            match current_stmt.decl_node.unwrap().var_type.unwrap().as_str() {
                "int" => {
                    assembly.stack_offset += 4;
                    decl_str = format!(
                        "{}\nmov dword[rbp-{}], eax",
                        expression_str, assembly.stack_offset
                    );
                }
                "bool" | "char" => {
                    assembly.stack_offset += 1;
                    decl_str = format!(
                        "{}\nmov byte[rbp-{}], al",
                        expression_str, assembly.stack_offset
                    );
                }
                "float" => {
                    assembly.stack_offset += 8;
                    decl_str = format!(
                        "{}\nmovsd qword[rbp-{}], xmm0",
                        expression_str, assembly.stack_offset
                    );
                }
                _ => {}
            }

            println!("{}", decl_str);
        }
        _ => {
            println!("{}", current_stmt.stmt_type)
        }
    }

    assembly
}

pub fn generate_assembly(ast: Stmt) {
    let mut assembly = Assembly::new(String::new(), String::new());
    recursive_gen(ast, assembly);
}
