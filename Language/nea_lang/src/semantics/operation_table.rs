use std::collections::HashMap;

pub fn load_operation_table() -> HashMap<(&'static str, &'static str, &'static str), &'static str> {
    HashMap::from([
        // No inter-type operations
        // e.g (int + float)
        (("Add", "Int", "Int"), "Int"),
        (("Add", "Float", "Float"), "Float"),
        (("Sub", "Int", "Int"), "Int"),
        (("Sub", "Float", "Float"), "Float"),
        (("Mul", "Int", "Int"), "Int"),
        (("Mul", "Float", "Float"), "Float"),
        (("Div", "Int", "Int"), "Float"),
        (("Div", "Float", "Float"), "Float"),
        (("Mod", "Int", "Int"), "Int"),
        (("Mod", "Float", "Float"), "Float"),
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
    ])
}
