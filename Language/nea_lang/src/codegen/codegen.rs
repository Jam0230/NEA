use crate::parser::parser::{Decl, Expr, Stmt};

#[derive(Clone, Debug)]
struct Symbol {
    name: String,
    _type: String,
    offset: i32,
}

impl Symbol {
    fn new(name: String, _type: String, offset: i32) -> Self {
        Self {
            name,
            _type,
            offset,
        }
    }
}

#[derive(Debug)]
struct Assembly {
    text_string: String, // the assembly code
    data_string: String, // stored data (floats, strings)
    stack_offset: i32,   // current offset being used for variables
    // (going to have to rethink this one)
    symbol_table: Vec<Symbol>, // table of symbols being used in the program
    float_num: i32,            // current number used for float labels
    jump_num: i32,             // current number used for jump point numbers
}

impl Assembly {
    fn new() -> Self {
        Self {
            text_string: String::new(),
            data_string: String::new(),
            stack_offset: 0,
            symbol_table: Vec::new(),
            float_num: 0,
            jump_num: 0,
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
    symbol_table: Vec<Symbol>,
    mut float_num: i32,
) -> (
    String,      /*Node str*/
    String,      /*Data Str*/
    Vec<String>, /*Used Regs*/
    i32,         /*Float num*/
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
                return (String::new(), String::new(), regs_used, float_num);
            }

            match node.expr_type.as_str() {
                "Bool" => {
                    if register.chars().last().unwrap() == 'x' {
                        register = format!("{}l", register.chars().nth(1).unwrap());
                    } else if register.chars().last().unwrap() == 'i' {
                        register = format!("{}l", register.replace("r", ""));
                    } else {
                        register += "b";
                    }
                }
                _ => {
                    if ['x', 'i'].contains(&register.chars().last().unwrap()) {
                        register = register.replace("r", "e");
                    } else {
                        register = format!("{}d", register);
                    }
                }
            }

            match node.val.clone().unwrap().as_str() {
                "true" => {
                    node_str = format!("mov {}, 1", register);
                }
                "false" => {
                    node_str = format!("mov {}, 0", register);
                }
                _ => {
                    node_str = format!("mov {}, {}", register, node.val.unwrap());
                }
            }
        }
        "Float" => {
            let mut register = String::new();
            for i in 0..=15 {
                if !regs_used.contains(&format!("xmm{}", i)) {
                    register = format!("xmm{}", i);
                    regs_used.push(register.clone());
                    break;
                }
            }

            node_str = format!("movsd {}, [f{}]", register, float_num);
            data_str = format!("f{}: dq {}", float_num, node.val.unwrap());
            float_num += 1;
        }
        "Id" => {
            let identifier = node.val.unwrap();
            let symbol = symbol_table.iter().find(|x| x.name == identifier);

            match symbol {
                Some(s) => match s._type.as_str() {
                    "int" | "bool" | "char" => {
                        let mut register = String::new();
                        for reg in usable_regs_int {
                            if !regs_used.contains(&String::from(reg)) {
                                register = String::from(reg);
                                regs_used.push(String::from(reg));
                                break;
                            }
                        }

                        if register == String::new() {
                            print!(
                                "Expression is too chunky :O\n Please make it shorter for me :3"
                            );
                            return (String::new(), String::new(), regs_used, float_num);
                        }

                        let data_length;
                        match s._type.as_str() {
                            "bool" => {
                                if register.chars().last().unwrap() == 'x' {
                                    register = format!("{}l", register.chars().nth(1).unwrap());
                                } else if register.chars().last().unwrap() == 'i' {
                                    register = format!("{}l", register.replace("r", ""));
                                } else {
                                    register += "b";
                                }
                                data_length = "byte";
                            }
                            _ => {
                                if ['x', 'i'].contains(&register.chars().last().unwrap()) {
                                    register = register.replace("r", "e");
                                } else {
                                    register = format!("{}d", register);
                                }
                                data_length = "dword";
                            }
                        }
                        node_str = format!("mov {}, {}[rbp-{}]", register, data_length, s.offset);
                    }
                    _ => {}
                },
                None => {} //TODO: Future error moment :3
            }
        }
        // ## Operations ##
        "Add" | "Sub" | "Mul" => {
            if let Some('r') = regs_used.last().unwrap().chars().nth(0) {
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let opcode;
                match node.expr_type.as_str() {
                    "Mul" => {
                        opcode = String::from("imul");
                    }
                    _ => {
                        opcode = node.expr_type.to_lowercase();
                    }
                }

                node_str = format!("{} {}, {}", opcode, operand_1, operand_2);
                regs_used.push(operand_1);
            } else {
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let opcode = format!("{}sd", node.expr_type.to_lowercase());

                node_str = format!("{} {},{}", opcode, operand_1, operand_2);
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
            } else {
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                node_str = format!("divsd {},{}", operand_1, operand_2);
                regs_used.push(operand_1);
            }
        }
        "Mod" => {
            if let Some('r') = regs_used.last().unwrap().chars().nth(0) {
                // if rax / rdx in use push to stack and pop after
                // put dividend (first one) in rax
                // put divisor in next availble register
                // div <divisor>
                // remainder stored in rdx moved to operand 1

                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let mut pushed_regs: Vec<String> = Vec::new();
                for reg in ["rax", "rdx"] {
                    if regs_used.contains(&String::from(reg)) {
                        node_str = format!("{}\npush {}", node_str, reg);
                        pushed_regs.push(String::from(reg));
                    }
                }

                let mut backup_reg: String = String::new();
                for reg in usable_regs_int {
                    if !(regs_used.contains(&String::from(reg)) || ["rax", "rdx"].contains(&reg)) {
                        backup_reg = String::from(reg);
                    }
                }

                if operand_2 == String::from("rdx") {
                    node_str = format!(
                        "{}\nmov rax, {}\n mov {}, rdx\nxor rdx, rdx\ndiv {}\nmov {}, rdx",
                        node_str, operand_1, backup_reg, backup_reg, operand_1
                    );
                } else {
                    node_str = format!(
                        "{}\nmov rax, {}\nxor rdx, rdx\ndiv {}\nmov {}, rdx",
                        node_str, operand_1, operand_2, operand_1
                    );
                }

                regs_used.push(operand_1);

                for reg in pushed_regs {
                    node_str = format!("{}\npop {}", node_str, reg);
                }
            } else {
            }
        }
        "LogAnd" | "LogOr" => {
            println!("{:?}", regs_used);
            let mut operand_2 = regs_used.pop().unwrap();
            let mut operand_1 = regs_used.pop().unwrap();
            regs_used.push(operand_1.clone());

            if operand_1.chars().last().unwrap() == 'x' {
                operand_1 = format!("{}l", operand_1.chars().nth(1).unwrap());
            } else if operand_1.chars().last().unwrap() == 'i' {
                operand_1 = format!("{}l", operand_1.replace("r", ""));
            } else {
                operand_1 += "b";
            }
            if operand_2.chars().last().unwrap() == 'x' {
                operand_2 = format!("{}l", operand_2.chars().nth(1).unwrap());
            } else if operand_2.chars().last().unwrap() == 'i' {
                operand_2 = format!("{}l", operand_2.replace("r", ""));
            } else {
                operand_2 += "b";
            }

            let opcode = node.expr_type.replace("Log", "").to_lowercase();

            node_str = format!("{} {}, {}", opcode, operand_1, operand_2);
        }
        "LogNot" => {
            let mut operand_1 = regs_used.pop().unwrap();
            regs_used.push(operand_1.clone());

            if operand_1.chars().last().unwrap() == 'x' {
                operand_1 = format!("{}l", operand_1.chars().nth(1).unwrap());
            } else if operand_1.chars().last().unwrap() == 'i' {
                operand_1 = format!("{}l", operand_1.replace("r", ""));
            } else {
                operand_1 += "b";
            }

            node_str = format!("xor {}, 1", operand_1);
        }
        "Eq" | "Neq" | "Lt" | "Gt" | "LtEq" | "GtEq" => {
            let set_type = format!(
                "set{}",
                node.expr_type
                    .to_lowercase()
                    .replace("q", "")
                    .replace("t", "")
            );

            if let Some('r') = regs_used.last().unwrap().chars().nth(0) {
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let mut return_reg: String = String::new();
                for reg in usable_regs_int {
                    if !regs_used.contains(&String::from(reg)) {
                        return_reg = String::from(reg);
                        regs_used.push(String::from(reg));
                        break;
                    }
                }

                if return_reg.chars().last().unwrap() == 'x' {
                    return_reg = format!("{}l", return_reg.chars().nth(1).unwrap());
                } else if operand_1.chars().last().unwrap() == 'i' {
                    return_reg = format!("{}l", return_reg.replace("r", ""));
                } else {
                    return_reg += "b";
                }

                node_str = format!(
                    "cmp {}, {}\n{} {}",
                    operand_1, operand_2, set_type, return_reg
                );
            } else {
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                let mut return_reg: String = String::new();
                for reg in usable_regs_int {
                    if !regs_used.contains(&String::from(reg)) {
                        return_reg = String::from(reg);
                        regs_used.push(String::from(reg));
                        break;
                    }
                }

                if return_reg.chars().last().unwrap() == 'x' {
                    return_reg = format!("{}l", return_reg.chars().nth(1).unwrap());
                } else if operand_1.chars().last().unwrap() == 'i' {
                    return_reg = format!("{}l", return_reg.replace("r", ""));
                } else {
                    return_reg += "b";
                }

                node_str = format!(
                    "ucomisd {}, {}\n{} {}",
                    operand_1, operand_2, set_type, return_reg
                );
            }
        }
        _ => {
            println!("{}", node.expr_type);
        }
    }
    (node_str, data_str, regs_used, float_num)
}

fn rpngen(
    expression: Expr,
    symbol_table: Vec<Symbol>,
    mut float_num: i32,
) -> (String, String, i32) {
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

        let (node_str, node_data_str, regs_used_temp, float_num_) =
            expression_node_gen(current_node, regs_used, symbol_table.clone(), float_num);
        regs_used = regs_used_temp;
        expression_str = format!("{}\n{}", expression_str, node_str);
        data_str = format!("{}\n{}", data_str, node_data_str);
        float_num = float_num_;

        if moves.len() == 0 {
            break;
        }

        if moves.pop().unwrap() == 0 {
            moves.push(1);
            skipleft = false;
        } else {
            skipleft = true;
        }
    }
    (expression_str, data_str, float_num)
}

/*
 ## Generating assembly ##

recursivly go through the ast adding the generated assembly to the correct string in the assembly struct (much like semantic analysis)
 - Normal code goes in "text_string"
 - Data for floating point and string constants in "data_string"
  */
fn recursive_gen(current_stmt: Stmt, mut assembly: Assembly) -> Assembly {
    match current_stmt.stmt_type.as_str() {
        // ## Statements ##
        "ElseStmt" => {
            // generate body
            match current_stmt.clone().body {
                Some(body) => assembly = recursive_gen(*body, assembly),
                None => {}
            }
        }
        "IfStmt" | "ElifStmt" => {
            // generate expression assembly for if stmt
            // (since boolean value can assume it will be 1 or 0 in al)
            // generate body of if stmt
            // generate other parts of stmt (elif's and else)

            let (expression_str, expr_data_str, float_num_) = rpngen(
                current_stmt.clone().expr.unwrap(),
                assembly.symbol_table.clone(),
                assembly.float_num,
            );
            assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
            assembly.float_num = float_num_;

            let jump_num = assembly.jump_num;
            assembly.jump_num += 1;

            assembly.text_string = format!(
                "{}\n{}\ncmp al, 1\njne .{}",
                assembly.text_string, expression_str, jump_num
            );

            match current_stmt.clone().body {
                Some(body) => assembly = recursive_gen(*body, assembly),
                None => {}
            }

            assembly.text_string = format!("{}\n.{}:", assembly.text_string, jump_num);

            match current_stmt.clone().elif_stmt {
                Some(elif_stmt) => assembly = recursive_gen(*elif_stmt, assembly),
                None => {}
            }
            match current_stmt.clone().else_stmt {
                Some(else_stmt) => assembly = recursive_gen(*else_stmt, assembly),
                None => {}
            }

            if current_stmt.stmt_type.as_str() == "IfStmt" {
                for i in jump_num..assembly.jump_num {
                    assembly.text_string = assembly.text_string.replace(
                        format!(".{}:", i).as_str(),
                        format!("jmp .{}\n.{}:", assembly.jump_num, i).as_str(),
                    );
                }
                assembly.text_string = format!("{}\n.{}:", assembly.text_string, assembly.jump_num);
            }
        }
        "WhileStmt" => {
            let jump_num_top = assembly.jump_num;
            let jump_num_bot = assembly.jump_num + 1;
            assembly.jump_num += 2;

            let (expression_str, expr_data_str, float_num_) = rpngen(
                current_stmt.clone().expr.unwrap(),
                assembly.symbol_table.clone(),
                assembly.float_num,
            );
            assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
            assembly.float_num = float_num_;

            assembly.text_string = format!(
                "{}\n\n.{}:{}\ncmp al, 1\n jne .{}",
                assembly.text_string, jump_num_top, expression_str, jump_num_bot
            );

            match current_stmt.clone().body {
                Some(body) => assembly = recursive_gen(*body, assembly),
                None => {}
            }

            assembly.text_string = format!(
                "{}\n\njmp .{}\n.{}:",
                assembly.text_string, jump_num_top, jump_num_bot
            );
        }
        // ## Variables ##
        "DeclStmt" => {
            let (expression_str, expr_data_str, float_num_) = rpngen(
                current_stmt.clone().decl_node.unwrap().value.unwrap(),
                assembly.symbol_table.clone(),
                assembly.float_num,
            );

            assembly.float_num = float_num_;

            let mut decl_str = String::new();

            match current_stmt
                .decl_node
                .clone()
                .unwrap()
                .var_type
                .unwrap()
                .as_str()
            {
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

            let symbol = Symbol::new(
                current_stmt.decl_node.clone().unwrap().id.unwrap(),
                current_stmt.decl_node.unwrap().var_type.unwrap(),
                assembly.stack_offset,
            );
            assembly.symbol_table.push(symbol);

            assembly.text_string = format!("{}\n{}", assembly.text_string, decl_str);
            assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
        }
        "AssignStmt" => {
            /*
               create new expression tree using stmts vaue tree and value being added too |

               add 2 new nodes:
                   - operation node (new)
                       l - identifier node (new)
                       r - stmts value

            */

            let stmt_value_expr = current_stmt.decl_node.clone().unwrap().value.unwrap();
            let assign_type = current_stmt.decl_node.clone().unwrap().ass_type.unwrap();
            let identifier = current_stmt.decl_node.clone().unwrap().id;
            let symbol = assembly
                .symbol_table
                .iter()
                .find(|x| x.name == identifier.clone().unwrap());

            let full_expr: Expr;

            match assign_type.as_str() {
                //TODO: Handle cases where "/=" is used (messes up due to
                //how types have been implemented for division operations)
                //might be a problem with semantic analysis
                "+=" => {
                    full_expr = Expr {
                        expr_type: String::from("Add"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    };
                }
                "-=" => {
                    full_expr = Expr {
                        expr_type: String::from("Sub"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    };
                }
                "*=" => {
                    full_expr = Expr {
                        expr_type: String::from("Mul"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    };
                }
                "=" => {
                    full_expr = stmt_value_expr;
                }
                _ => {
                    // TODO: You could count this as an error but i mean :p
                    full_expr = Expr {
                        expr_type: String::from("Add"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    };
                }
            }

            let (expression_str, expr_data_str, float_num_) =
                rpngen(full_expr, assembly.symbol_table.clone(), assembly.float_num);

            assembly.float_num = float_num_;

            let mut assign_str = String::new();

            println!("{}", symbol.unwrap()._type);

            match symbol.unwrap()._type.as_str() {
                "int" => {
                    assign_str = format!(
                        "{}\nmov dword[rbp-{}], eax",
                        expression_str,
                        symbol.unwrap().offset
                    );
                }
                "bool" | "char" => {
                    assign_str = format!(
                        "{}\nmov byte[rbp-{}], al",
                        expression_str,
                        symbol.unwrap().offset
                    );
                }
                "float" => {
                    assign_str = format!(
                        "{}\nmovsd qword[rbp-{}], xmm0",
                        expression_str,
                        symbol.unwrap().offset
                    );
                }
                _ => {}
            }

            assembly.text_string = format!("{}\n{}", assembly.text_string, assign_str);
            assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
        }
        _ => {
            println!("{}", current_stmt.stmt_type)
        }
    }

    match current_stmt.next {
        Some(stmt) => {
            assembly = recursive_gen(*stmt, assembly);
            return assembly;
        }
        None => return assembly,
    }
}

pub fn generate_assembly(ast: Stmt) {
    println!(" ---- ASSEMBLY GENERATION  -----");
    let mut assembly = Assembly::new();
    assembly = recursive_gen(ast, assembly);

    println!("code:{}", assembly.text_string);
    println!("\ndata:{}", assembly.data_string);
}
