use crate::parser::parser::{Expr, Stmt};

#[derive(Clone, Debug)]
pub struct Symbol {
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

#[derive(Clone, Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }

    pub fn search_for_symbol(self, symbol: String) -> Option<Symbol> {
        self.symbols.iter().find(|x| x.name == symbol).cloned()
    }
}

#[derive(Debug)]
pub struct Assembly {
    pub text_string: String,         // the assembly code
    pub data_string: String,         // stored data (floats, strings)
    pub stack_offset: i32,           // current offset being used for variables
    symbol_tables: Vec<SymbolTable>, // table symbol tables
    float_num: i32,                  // current number used for float labels
    jump_num: i32,                   // current number used for jump point numbers
}

impl Assembly {
    fn new() -> Self {
        // Initialises a new Assembly object
        Self {
            text_string: String::new(),
            data_string: String::new(),
            stack_offset: 0,
            symbol_tables: vec![SymbolTable::new()],
            float_num: 0,
            jump_num: 0,
        }
    }
}

fn enter_local_scope(current_stmt: Stmt, mut assembly: Assembly) -> Assembly {
    // push new symbol table to symbol table stack
    assembly.symbol_tables.push(SymbolTable::new());

    // generate new scope
    assembly = recursive_gen(current_stmt, assembly);

    // remove new symbol table
    assembly.symbol_tables.pop();
    assembly
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
    symbol_tables: Vec<SymbolTable>,
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
            // find next unused register
            let mut register = String::new();
            for reg in usable_regs_int {
                if !regs_used.contains(&String::from(reg)) {
                    register = String::from(reg);
                    regs_used.push(String::from(reg));
                    break;
                }
            }

            // incase no registers are available
            if register == String::new() {
                print!("Expression is too chunky :O\n Please make it shorter for me");
                return (String::new(), String::new(), regs_used, float_num);
            }

            // change register size for differing literals
            // Bool => 1B
            // int/char => 4B
            match node.expr_type.as_str() {
                "Bool" => {
                    if register.ends_with('x') {
                        register = format!("{}l", register.chars().nth(1).unwrap());
                    } else if register.ends_with('i') {
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

            // boolean values changed to 1/0
            // others can be put in litteraly
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
            // find next unused xmm~ register
            let mut register = String::new();
            for i in 0..=15 {
                if !regs_used.contains(&format!("xmm{}", i)) {
                    register = format!("xmm{}", i);
                    regs_used.push(register.clone());
                    break;
                }
            }

            // add code to assembly object
            node_str = format!("movsd {}, [f{}]", register, float_num);
            data_str = format!("f{}: dq {}", float_num, node.val.unwrap());
            float_num += 1;
        }
        "Id" => {
            // search symbol table
            let identifier = node.val.unwrap();
            let mut symbol: Option<Symbol> = None;
            for symboltable in symbol_tables.iter().rev() {
                symbol = symboltable.clone().search_for_symbol(identifier.clone());
            }

            // let symbol = symbol_table.iter().find(|x| x.name == identifier);

            if let Some(s) = symbol {
                // if symbol exists
                match s._type.as_str() {
                    "int" | "bool" | "char" => {
                        // find next unused register
                        let mut register = String::new();
                        for reg in usable_regs_int {
                            if !regs_used.contains(&String::from(reg)) {
                                register = String::from(reg);
                                regs_used.push(String::from(reg));
                                break;
                            }
                        }

                        // if no registers exist
                        if register == String::new() {
                            // could push extra operands to the stack
                            print!("Expression is too chunky :O\n Please make it shorter for me");
                            return (String::new(), String::new(), regs_used, float_num);
                        }

                        // change data length based on typing
                        // bool => 1B
                        // other => 4B
                        let data_length;
                        match s._type.as_str() {
                            "bool" => {
                                if register.ends_with('x') {
                                    register = format!("{}l", register.chars().nth(1).unwrap());
                                } else if register.ends_with('i') {
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
                    _ => {} // if symbol doesnt exist (shouldnt happen due to semantic analysis)
                }
            }
        }
        // ## Operations ##
        "Add" | "Sub" | "Mul" => {
            // get the last two registers used from the register stack
            let operand_2 = regs_used.pop().unwrap();
            let operand_1 = regs_used.pop().unwrap();

            let opcode = if let Some('r') = operand_2.chars().next() {
                // if operation between two integer types
                // Mul => imul (nasm stuff)
                match node.expr_type.as_str() {
                    "Mul" => String::from("imul"),
                    _ => node.expr_type.to_lowercase(),
                }
            } else {
                // if operation between two floating point types
                format!("{}sd", node.expr_type.to_lowercase())
            };

            // add code to assembly object
            node_str = format!("{} {},{}", opcode, operand_1, operand_2);
            regs_used.push(operand_1);
        }
        "Div" => {
            // collect last two register used from register stack
            let operand_2 = regs_used.pop().unwrap();
            let operand_1 = regs_used.pop().unwrap();

            if let Some('r') = operand_2.chars().next() {
                // if integer Operations

                // find next two unused xmm~ registers
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

                // convert integers into floating point numbers before operation
                node_str = format!(
                    "cvtsi2sd {reg_1}, {op_1}\ncvtsi2sd {reg_2}, {op_2}\ndivsd {reg_1}, {reg_2}",
                    op_1 = operand_1,
                    op_2 = operand_2,
                    reg_1 = register_1,
                    reg_2 = register_2
                );
                regs_used.push(register_1);
            } else {
                // if floating point operation

                node_str = format!("divsd {},{}", operand_1, operand_2);
                regs_used.push(operand_1);
            }
        }
        "Mod" => {
            if let Some('r') = regs_used.last().unwrap().chars().next() {
                // if integer modulo

                // collect last two registers used from the register stack
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                // push rax/rdx if they are used
                // as they are destroyed in the division operation
                let mut pushed_regs: Vec<String> = Vec::new();
                for reg in ["rax", "rdx"] {
                    if regs_used.contains(&String::from(reg)) {
                        node_str = format!("{}\npush {}", node_str, reg);
                        pushed_regs.push(String::from(reg));
                    }
                }

                // find next unused register to use incase rdx is used during operation
                let mut backup_reg: String = String::new();
                for reg in usable_regs_int {
                    if !(regs_used.contains(&String::from(reg)) || ["rax", "rdx"].contains(&reg)) {
                        backup_reg = String::from(reg);
                    }
                }

                // perform operation
                if operand_2 == *"rdx" {
                    node_str = format!(
                        "{}\nmov rax, {}\nmov {}, rdx\nxor rdx, rdx\ndiv {}\nmov {}, rdx",
                        node_str, operand_1, backup_reg, backup_reg, operand_1
                    );
                } else {
                    node_str = format!(
                        "{}\nmov rax, {}\nxor rdx, rdx\ndiv {}\nmov {}, rdx",
                        node_str, operand_1, operand_2, operand_1
                    );
                }

                regs_used.push(operand_1);

                // restore rax/rdx
                for reg in pushed_regs.iter().rev() {
                    node_str = format!("{}\npop {}", node_str, reg);
                }
            } else {
                // if floating point modulo

                // find next unused xmm~ register
                let mut operand_3 = String::new();
                for i in 0..=15 {
                    if !(regs_used.contains(&format!("xmm{}", i))) {
                        operand_3 = format!("xmm{}", i);
                        break;
                    }
                }

                // get last two registers used from register stack
                let operand_2 = regs_used.pop().unwrap();
                let operand_1 = regs_used.pop().unwrap();

                // Equation sourced from: https://stackoverflow.com/questions/9505513/floating-point-modulo-operation
                // V % M = V - trunc(V/M) * M
                // Innacurate for larger numbers but :P

                node_str = format!(
                    "movsd {trunc}, {value}\n
divsd {trunc}, {modulus}\n
push rax\n
cvttsd2si rax, {trunc}\n
cvtsi2sd {trunc}, rax\n
pop rax\n
mulsd {trunc}, {modulus}\n
subsd {value}, {trunc}",
                    value = operand_1,
                    modulus = operand_2,
                    trunc = operand_3
                );
                regs_used.push(operand_1);
            }
        }
        "LogAnd" | "LogOr" => {
            // get last two used registers from register stack
            let mut operand_2 = regs_used.pop().unwrap();
            let mut operand_1 = regs_used.pop().unwrap();
            regs_used.push(operand_1.clone());

            // formatting registers to get just the last Byte
            if operand_1.ends_with('x') {
                operand_1 = format!("{}l", operand_1.chars().nth(1).unwrap());
            } else if operand_1.ends_with('i') {
                operand_1 = format!("{}l", operand_1.replace("r", ""));
            } else {
                operand_1 += "b";
            }
            if operand_2.ends_with('x') {
                operand_2 = format!("{}l", operand_2.chars().nth(1).unwrap());
            } else if operand_2.ends_with('i') {
                operand_2 = format!("{}l", operand_2.replace("r", ""));
            } else {
                operand_2 += "b";
            }

            let opcode = node.expr_type.replace("Log", "").to_lowercase();

            node_str = format!("{} {}, {}", opcode, operand_1, operand_2);
        }
        "LogNot" => {
            // get last used register from register stack
            let mut operand_1 = regs_used.pop().unwrap();
            regs_used.push(operand_1.clone());

            // formatting registers to get just the last Byte
            if operand_1.ends_with('x') {
                operand_1 = format!("{}l", operand_1.chars().nth(1).unwrap());
            } else if operand_1.ends_with('i') {
                operand_1 = format!("{}l", operand_1.replace("r", ""));
            } else {
                operand_1 += "b";
            }

            node_str = format!("xor {}, 1", operand_1);
        }
        "Eq" | "Neq" | "Lt" | "Gt" | "LtEq" | "GtEq" => {
            // set the comparison type
            let set_type = format!(
                "set{}",
                node.expr_type
                    .to_lowercase()
                    .replace("q", "")
                    .replace("t", "")
            );

            // get last two used registers from register stack
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

            if return_reg.ends_with('x') {
                return_reg = format!("{}l", return_reg.chars().nth(1).unwrap());
            } else if operand_1.ends_with('i') {
                return_reg = format!("{}l", return_reg.replace("r", ""));
            } else {
                return_reg += "b";
            }

            if let Some('r') = operand_2.chars().next() {
                // comparison between two integer types
                node_str = format!(
                    "cmp {}, {}\n{} {}",
                    operand_1, operand_2, set_type, return_reg
                );
            } else {
                // comparison between two floating point types
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
    symbol_tables: Vec<SymbolTable>,
    mut float_num: i32,
) -> (String, String, i32) {
    let mut moves = vec![0]; // moves to follow
    let mut skipleft = false; // flag to skip moving to the left
    let mut expression_str = String::new(); // output code string for the expression
    let mut data_str = String::new(); // ouput data string for the expression

    let mut regs_used: Vec<String> = Vec::new(); // registers used in the expression

    loop {
        // follow the moves in <moves>
        // 0 = left
        // 1 = right
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

        // keep moving left until no more nodes on the left
        // or if <skipleft flag is set>
        while current_node.left.is_some() && !skipleft {
            current_node = *current_node.left.unwrap();
            moves.push(0);
        }

        // generate the node
        let (node_str, node_data_str, regs_used_temp, float_num_) =
            expression_node_gen(current_node, regs_used, symbol_tables.clone(), float_num);
        regs_used = regs_used_temp;
        expression_str = format!("{}\n{}", expression_str, node_str);
        data_str = format!("{}\n{}", data_str, node_data_str);
        float_num = float_num_;

        // if at the end of the expression
        if moves.is_empty() {
            break;
        }

        if moves.pop().unwrap() == 0 {
            // if last move was a left
            // replace with a right
            moves.push(1);
            skipleft = false;
        } else {
            // if last move was a right
            // remove move and skip left next iteration
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
fn recursive_gen(mut current_stmt: Stmt, mut assembly: Assembly) -> Assembly {
    loop {
        println!("{:#?}", assembly);
        match current_stmt.stmt_type.as_str() {
            // ## Statements ##
            "ElseStmt" => {
                // generate body
                if let Some(body) = current_stmt.clone().body {
                    assembly = enter_local_scope(*body, assembly)
                }
            }
            "IfStmt" | "ElifStmt" => {
                // generate expression assembly for if stmt
                // (since boolean value can assume it will be 1 or 0 in al)
                // generate body of if stmt
                // generate other parts of stmt (elif's and else)

                // generate condition expression
                let (expression_str, expr_data_str, float_num_) = rpngen(
                    current_stmt.clone().expr.unwrap(),
                    assembly.symbol_tables.clone(),
                    assembly.float_num,
                );
                assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
                assembly.float_num = float_num_;

                let jump_num = assembly.jump_num;
                assembly.jump_num += 1;

                // generate condition
                assembly.text_string = format!(
                    "{}\n{}\ncmp al, 1\njne .{}",
                    assembly.text_string, expression_str, jump_num
                );

                // generate body
                if let Some(body) = current_stmt.clone().body {
                    assembly = enter_local_scope(*body, assembly)
                }

                // add jump point for false
                assembly.text_string = format!("{}\n.{}:", assembly.text_string, jump_num);

                // generate elif statements
                if let Some(elif_stmt) = current_stmt.clone().stmt_1 {
                    assembly = recursive_gen(*elif_stmt, assembly)
                }

                // generate else statement
                if let Some(else_stmt) = current_stmt.clone().stmt_2 {
                    assembly = recursive_gen(*else_stmt, assembly)
                }

                // change all jump points for finished bodies from the if/elif/else stmts to point
                // to final jump point of the if stmt
                if current_stmt.stmt_type.as_str() == "IfStmt" {
                    for i in jump_num..assembly.jump_num {
                        assembly.text_string = assembly.text_string.replace(
                            format!(".{}:", i).as_str(),
                            format!("jmp .{}\n.{}:", assembly.jump_num, i).as_str(),
                        );
                    }
                    // add the final jump point of the if stmt
                    assembly.text_string =
                        format!("{}\n.{}:", assembly.text_string, assembly.jump_num);
                }
            }
            "WhileStmt" => {
                let jump_num_top = assembly.jump_num;
                let jump_num_bot = assembly.jump_num + 1;
                assembly.jump_num += 2;

                // generate condition expression
                let (expression_str, expr_data_str, float_num_) = rpngen(
                    current_stmt.clone().expr.unwrap(),
                    assembly.symbol_tables.clone(),
                    assembly.float_num,
                );
                assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
                assembly.float_num = float_num_;

                // generate condition
                assembly.text_string = format!(
                    "{}\n\n.{}:{}\ncmp al, 1\n jne .{}",
                    assembly.text_string, jump_num_top, expression_str, jump_num_bot
                );

                // generate body
                if let Some(body) = current_stmt.clone().body {
                    assembly = enter_local_scope(*body, assembly);
                }

                // add last jump point for loop
                assembly.text_string = format!(
                    "{}\n\njmp .{}\n.{}:",
                    assembly.text_string, jump_num_top, jump_num_bot
                );
            }
            "ForStmt" => {
                // generate decleration stmt
                // generate condition stmt
                // generate body
                // jmp point and repeat stmt at the end

                let jump_num_top = assembly.jump_num;
                let jump_num_bot = assembly.jump_num + 1;
                assembly.jump_num += 2;

                // generate control variable
                assembly = recursive_gen(*current_stmt.clone().stmt_1.unwrap(), assembly);

                // generate condition
                let (condition_expr_string, condition_expr_data_str, float_num_) = rpngen(
                    current_stmt.clone().expr.unwrap(),
                    assembly.symbol_tables.clone(),
                    assembly.float_num,
                );

                assembly.data_string =
                    format!("{}\n{}", assembly.data_string, condition_expr_data_str);
                assembly.float_num = float_num_;

                assembly.text_string = format!(
                    "{}\n.{}:{}\ncmp al,1\njne .{}",
                    assembly.text_string, jump_num_top, condition_expr_string, jump_num_bot
                );

                // generate body
                if let Some(body) = current_stmt.clone().body {
                    assembly = enter_local_scope(*body, assembly);
                }

                // generate increment stmt
                assembly = recursive_gen(*current_stmt.clone().stmt_2.unwrap(), assembly);

                assembly.text_string = format!(
                    "{}\njmp .{}\n.{}:",
                    assembly.text_string, jump_num_top, jump_num_bot
                );
            }
            // ## Variables ##
            "DeclStmt" => {
                // generate expression for decl stmt
                let (expression_str, expr_data_str, float_num_) = rpngen(
                    current_stmt.clone().decl_node.unwrap().value.unwrap(),
                    assembly.symbol_tables.clone(),
                    assembly.float_num,
                );

                assembly.float_num = float_num_;

                let mut decl_str = String::new();

                // assign new value to variable
                // different types use different data lengths
                // int = 4B
                // bool/char = 1B
                // float = 8B
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

                // create new symbol and add it to the symbol table
                let symbol = Symbol::new(
                    current_stmt.decl_node.clone().unwrap().id.unwrap(),
                    current_stmt.decl_node.unwrap().var_type.unwrap(),
                    assembly.stack_offset,
                );
                assembly
                    .symbol_tables
                    .last_mut()
                    .unwrap()
                    .symbols
                    .push(symbol);

                assembly.text_string = format!("{}\n{}", assembly.text_string, decl_str);
                assembly.data_string = format!("{}\n{}", assembly.data_string, expr_data_str);
            }
            "AssignStmt" => {
                // breaking up assign stmt data
                let stmt_value_expr = current_stmt.decl_node.clone().unwrap().value.unwrap();
                let assign_type = current_stmt.decl_node.clone().unwrap().ass_type.unwrap();
                let identifier = current_stmt.decl_node.clone().unwrap().id;

                // finding symbol in the symbol table
                let mut symbol: Option<Symbol> = None;
                for symboltable in assembly.symbol_tables.iter().rev() {
                    symbol = symboltable
                        .clone()
                        .search_for_symbol(identifier.clone().unwrap());
                }

                // create new expression tree's for full assign expression
                // e.g: x+=1 => x = x + 1
                let full_expr: Expr = match assign_type.as_str() {
                    "+=" => Expr {
                        expr_type: String::from("Add"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    },
                    "-=" => Expr {
                        expr_type: String::from("Sub"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    },
                    "*=" => Expr {
                        expr_type: String::from("Mul"),
                        left: Some(Box::new(Expr {
                            expr_type: String::from("Id"),
                            left: None,
                            right: None,
                            val: identifier,
                        })),
                        right: Some(Box::new(stmt_value_expr)),
                        val: None,
                    },
                    "=" => stmt_value_expr,
                    _ => {
                        // TODO: You could count this as an error but i mean :p
                        Expr {
                            expr_type: String::from("Add"),
                            left: Some(Box::new(Expr {
                                expr_type: String::from("Id"),
                                left: None,
                                right: None,
                                val: identifier,
                            })),
                            right: Some(Box::new(stmt_value_expr)),
                            val: None,
                        }
                    }
                };

                // generate new expression
                let (expression_str, expr_data_str, float_num_) = rpngen(
                    full_expr,
                    assembly.symbol_tables.clone(),
                    assembly.float_num,
                );

                assembly.float_num = float_num_;

                let mut assign_str = String::new();

                // generate assignment code
                // different types use different sizes
                // int = 4B
                // bool/char = 1B
                // float = 8B
                match symbol.clone().unwrap()._type.as_str() {
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
            _ => {}
        }

        // continue loop with next expression or return complete assembly code
        if current_stmt.next.is_some() {
            current_stmt = *current_stmt.next.unwrap();
        } else {
            return assembly;
        }
    }
}

pub fn generate_assembly(ast: Stmt) -> Assembly {
    recursive_gen(ast, Assembly::new())
}
