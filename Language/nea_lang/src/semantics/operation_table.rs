use std::collections::HashMap;
use std::fs;

pub fn parse_table() {
    let data = fs::read_to_string("/home/jam/Documents/School/ComputerScience/NEA/Language/nea_lang/res/tables/operation_table").expect("AHHHHHH");
    let lines = data.split('\n').collect::<Vec<&str>>();

    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let (condition, result) = (
            line.split("=>").collect::<Vec<&str>>()[0]
                .split(",")
                .collect::<Vec<&str>>(),
            line.split("=>").collect::<Vec<&str>>()[1],
        );

        println!(
            "(({:?},{:?},{:?}), {:?}),",
            condition[0], condition[1], condition[2], result
        );
        if condition[1] != condition[2] {
            println!(
                "(({:?},{:?},{:?}), {:?}),",
                condition[0], condition[2], condition[1], result
            );
        }
    }
}

pub fn load_operation_table() -> HashMap<(&'static str, &'static str, &'static str), &'static str> {
    let hash = HashMap::from([
        (("Add", "Int", "Int"), "Int"),
        (("Add", "Float", "Float"), "Float"),
        (("Add", "Int", "Float"), "Float"),
        (("Add", "Float", "Int"), "Float"),
        (("Sub", "Int", "Int"), "Int"),
        (("Sub", "Float", "Float"), "Float"),
        (("Sub", "Int", "Float"), "Float"),
        (("Sub", "Float", "Int"), "Float"),
        (("Mul", "Int", "Int"), "Int"),
        (("Mul", "Float", "Float"), "Float"),
        (("Mul", "Int", "Float"), "Float"),
        (("Mul", "Float", "Int"), "Float"),
        (("Div", "Int", "Int"), "Float"),
        (("Div", "Float", "Float"), "Float"),
        (("Div", "Int", "Float"), "Float"),
        (("Div", "Float", "Int"), "Float"),
        (("Eq", "Int", "Int"), "Bool"),
        (("Eq", "Float", "Float"), "Bool"),
        (("Eq", "Str", "Str"), "Bool"),
        (("Eq", "Bool", "Bool"), "Bool"),
        (("Eq", "Char", "Char"), "Bool"),
        (("Neq", "Int", "Int"), "Bool"),
        (("Neq", "Float", "Float"), "Bool"),
        (("Neq", "Str", "Str"), "Bool"),
        (("Neq", "Bool", "Bool"), "Bool"),
        (("Neq", "Char", "Char"), "Bool"),
        (("Lt", "Int", "Int"), "Bool"),
        (("Lt", "Float", "Float"), "Bool"),
        (("Lt", "Str", "Str"), "Bool"),
        (("Lt", "Bool", "Bool"), "Bool"),
        (("Lt", "Char", "Char"), "Bool"),
        (("Gt", "Int", "Int"), "Bool"),
        (("Gt", "Float", "Float"), "Bool"),
        (("Gt", "Str", "Str"), "Bool"),
        (("Gt", "Bool", "Bool"), "Bool"),
        (("Gt", "Char", "Char"), "Bool"),
        (("LtEq", "Int", "Int"), "Bool"),
        (("LtEq", "Float", "Float"), "Bool"),
        (("LtEq", "Str", "Str"), "Bool"),
        (("LtEq", "Bool", "Bool"), "Bool"),
        (("LtEq", "Char", "Char"), "Bool"),
        (("GtEq", "Int", "Int"), "Bool"),
        (("GtEq", "Float", "Float"), "Bool"),
        (("GtEq", "Str", "Str"), "Bool"),
        (("GtEq", "Bool", "Bool"), "Bool"),
        (("GtEq", "Char", "Char"), "Bool"),
        (("LogOr", "Bool", "Bool"), "Bool"),
        (("LogAnd", "Bool", "Bool"), "Bool"),
    ]);
    hash
}
