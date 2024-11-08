use crate::parser::parser::{Decl, Expr, Stmt};

fn expr_node_gen(
    node: Expr,
    mut regs_used: (Vec<String>, Vec<String>),
) -> (Vec<String>, Vec<String>) {
    /*
    code snippet for each node type
    when loading a value into a register add it to the the regs_used
    when doing an operation use the last two registers in the regs_used
    */

    let usable_regs = ["rax", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11"];
    println!("{:?}", regs_used);

    match node.expr_type.as_str() {
        "Int" => {
            // find next available register
            let mut used_register = String::from("");
            for reg in usable_regs {
                if !regs_used.0.contains(&String::from(reg)) {
                    used_register = String::from(reg);
                    regs_used.0.push(String::from(reg));
                    break;
                }
            }

            if used_register == "" {
                //TODO: Temp error
                println!(
                    "No registers available :O\nplease slim down this equation so I can do it :3"
                );
                return regs_used;
            }

            match used_register.chars().last().unwrap() {
                'x' | 'i' => used_register = format!("e{}", &used_register[1..]),
                _ => {
                    used_register = format!("{}d", used_register);
                }
            }

            let value: i32 = node.val.unwrap().parse::<i32>().unwrap();

            //TODO: change this to actually add the assembly to a file
            println!("mov {}, {}", used_register, value);
        }
        _ => println!("{}", node.expr_type),
    }

    return regs_used;
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

fn rpngen(expression: Expr) {
    let mut moves = vec![0];
    let mut skipleft = false;

    let mut regs_used: (Vec<String>, Vec<String>) = (Vec::new(), Vec::new()); // (integer regs,
                                                                              // floating point)

    while moves.len() >= 1 {
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

        println!("\nGenerating {}", current_node.expr_type);
        regs_used = expr_node_gen(current_node, regs_used);

        if moves.last().unwrap() == &0 {
            moves.pop();
            moves.push(1);
            skipleft = false;
        } else {
            moves.pop();
            skipleft = true;
        }

        if moves.len() == 0 {
            println!("\nGenerating {}", expression.expr_type);
        }
    }
}

pub fn test(statement: Stmt) {
    let expression = statement.decl_node.unwrap().value.unwrap();
    rpngen(expression);
}
