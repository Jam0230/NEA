use std::collections::HashMap;
use std::fs;

// used to generate parse table hash map
// #[allow(dead_code)]
// pub fn generate_parse_table() {
//     let parse_table_data = fs::read_to_string(
//         "/home/jam/Documents/School/ComputerScience/NEA/Language/nea_lang/res/tables/parse_table",
//     )
//     .expect("AHHHHHHHH but parsing :O");
//     let rows = parse_table_data.split(";;").collect::<Vec<&str>>();
//
//     for row in rows {
//         let lines = row.split('\n').collect::<Vec<&str>>();
//         let left_nt = format!("<{}>", lines[0].split('=').next().unwrap());
//
//         for line_i in 1..lines.len() - 2 {
//             let parts = lines[line_i].split("->").collect::<Vec<&str>>();
//             let input_expected = parts[0].trim();
//
//             let mut output = parts[1].split(',').collect::<Vec<&str>>();
//             output.retain(|x| x != &"");
//             let coll_token = format!("|{},{}|", output.len(), left_nt);
//
//             output.push(coll_token.as_str());
//             println!("(({:?}, {:?}), vec!{:?}),", left_nt, input_expected, output);
//         }
//     }
// }

pub fn load_parse_table() -> HashMap<(&'static str, &'static str), Vec<&'static str>> {
    // <_> = non-terminal
    // [_] = terminal type
    // |_,_| = collection token (used in parsing to generate ast)
    let hash = HashMap::from([
        (
            ("<SS>", "let"),
            vec!["<S>", "<SS>", "|Stmt-DeclStmt,2,2____1|"],
        ),
        (
            ("<SS>", "[Id]"),
            vec!["<S>", "<SS>", "|Stmt-AssignStmt,2,2____1|"],
        ),
        (
            ("<SS>", "if"),
            vec!["<S>", "<SS>", "|Stmt-IfStmt,8,_75321|"],
        ),
        (
            ("<SS>", "while"),
            vec!["<S>", "<SS>", "|Stmt-WhileStmt,6,_53__1|"],
        ),
        (("<SS>", "}"), vec!["|Stmt-None,0,______|"]),
        (("<SS>", "$"), vec!["|Stmt-None,0,______|"]),
        (("<S>", "let"), vec!["<DS>", ";", "|Decl-VarDecl,6,452_|"]),
        (
            ("<S>", "[Id]"),
            vec!["<AS>", ";", "|Decl-VarAssign,4,4_23|"],
        ),
        (("<S>", "if"), vec!["<IS>"]),
        (("<S>", "while"), vec!["<WS>"]),
        (("<DS>", "let"), vec!["let", "[Type]", "[Id]", "=", "<OE>"]),
        (("<AS>", "[Id]"), vec!["[Id]", "[Assignment]", "<OE>"]),
        (
            ("<IS>", "if"),
            vec!["if", "<OE>", "{", "<SS>", "}", "<EISP>", "<ES>"],
        ),
        (
            ("<EISP>", "elif"),
            vec!["<EIS>", "<EISP>", "|Stmt-ElifStmt,6,_53__1|"],
        ),
        (("<EISP>", "else"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "let"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "[Id]"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "if"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "while"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "}"), vec!["|Stmt-None,0,______|"]),
        (("<EISP>", "$"), vec!["|Stmt-None,0,______|"]),
        (("<EIS>", "elif"), vec!["elif", "<OE>", "{", "<SS>", "}"]),
        (
            ("<ES>", "else"),
            vec!["else", "{", "<SS>", "}", "|Stmt-ElseStmt,4,__2___|"],
        ),
        (("<ES>", "let"), vec!["|Stmt-None,0,______|"]),
        (("<ES>", "[Id]"), vec!["|Stmt-None,0,______|"]),
        (("<ES>", "if"), vec!["|Stmt-None,0,______|"]),
        (("<ES>", "while"), vec!["|Stmt-None,0,______|"]),
        (("<ES>", "}"), vec!["|Stmt-None,0,______|"]),
        (("<ES>", "$"), vec!["|Stmt-None,0,______|"]),
        (("<WS>", "while"), vec!["while", "<OE>", "{", "<SS>", "}"]),
        (("<OE>", "!"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[Int]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[Float]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[String]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[Char]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[Bool]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "[Id]"), vec!["<AE>", "<OEP>"]),
        (("<OE>", "("), vec!["<AE>", "<OEP>"]),
        (
            ("<OEP>", "||"),
            vec!["||", "<AE>", "|Expr-LogOr,3,31_|", "<OEP>"],
        ),
        (("<OEP>", ";"), vec![]),
        (("<OEP>", ")"), vec![]),
        (("<OEP>", "{"), vec![]),
        (("<AE>", "!"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[Int]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[Float]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[String]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[Char]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[Bool]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "[Id]"), vec!["<EE>", "<AEP>"]),
        (("<AE>", "("), vec!["<EE>", "<AEP>"]),
        (
            ("<AEP>", "&&"),
            vec!["&&", "<EE>", "|Expr-LogAnd,3,31_|", "<AEP>"],
        ),
        (("<AEP>", "||"), vec![]),
        (("<AEP>", ";"), vec![]),
        (("<AEP>", ")"), vec![]),
        (("<AEP>", "{"), vec![]),
        (("<EE>", "!"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[Int]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[Float]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[String]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[Char]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[Bool]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "[Id]"), vec!["<IE>", "<EEP>"]),
        (("<EE>", "("), vec!["<IE>", "<EEP>"]),
        (
            ("<EEP>", "=="),
            vec!["==", "<IE>", "|Expr-Eq,3,31_|", "<EEP>"],
        ),
        (
            ("<EEP>", "!="),
            vec!["!=", "<IE>", "|Expr-Neq,3,31_|", "<EEP>"],
        ),
        (("<EEP>", "&&"), vec![]),
        (("<EEP>", "||"), vec![]),
        (("<EEP>", ";"), vec![]),
        (("<EEP>", ")"), vec![]),
        (("<EEP>", "{"), vec![]),
        (("<IE>", "!"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[Int]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[Float]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[String]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[Char]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[Bool]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "[Id]"), vec!["<ADE>", "<IEP>"]),
        (("<IE>", "("), vec!["<ADE>", "<IEP>"]),
        (
            ("<IEP>", ">"),
            vec![">", "<ADE>", "|Expr-Gt,3,31_|", "<IEP>"],
        ),
        (
            ("<IEP>", "<"),
            vec!["<", "<ADE>", "|Expr-Lt,3,31_|", "<IEP>"],
        ),
        (
            ("<IEP>", ">="),
            vec![">=", "<ADE>", "|Expr-GtEq,3,31_|", "<IEP>"],
        ),
        (
            ("<IEP>", "<="),
            vec!["<=", "<ADE>", "|Expr-LtEq,3,31_|", "<IEP>"],
        ),
        (("<IEP>", "=="), vec![]),
        (("<IEP>", "!="), vec![]),
        (("<IEP>", "&&"), vec![]),
        (("<IEP>", "||"), vec![]),
        (("<IEP>", ";"), vec![]),
        (("<IEP>", ")"), vec![]),
        (("<IEP>", "{"), vec![]),
        (("<ADE>", "!"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[Int]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[Float]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[String]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[Char]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[Bool]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "[Id]"), vec!["<ME>", "<ADEP>"]),
        (("<ADE>", "("), vec!["<ME>", "<ADEP>"]),
        (
            ("<ADEP>", "+"),
            vec!["+", "<ME>", "|Expr-Add,3,31_|", "<ADEP>"],
        ),
        (
            ("<ADEP>", "-"),
            vec!["-", "<ME>", "|Expr-Sub,3,31_|", "<ADEP>"],
        ),
        (("<ADEP>", ">"), vec![]),
        (("<ADEP>", "<"), vec![]),
        (("<ADEP>", ">="), vec![]),
        (("<ADEP>", "<="), vec![]),
        (("<ADEP>", "=="), vec![]),
        (("<ADEP>", "!="), vec![]),
        (("<ADEP>", "&&"), vec![]),
        (("<ADEP>", "||"), vec![]),
        (("<ADEP>", ";"), vec![]),
        (("<ADEP>", ")"), vec![]),
        (("<ADEP>", "{"), vec![]),
        (("<ME>", "!"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[Int]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[Float]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[String]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[Char]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[Bool]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "[Id]"), vec!["<UE>", "<MEP>"]),
        (("<ME>", "("), vec!["<UE>", "<MEP>"]),
        (
            ("<MEP>", "*"),
            vec!["*", "<UE>", "|Expr-Mul,3,31_|", "<MEP>"],
        ),
        (
            ("<MEP>", "/"),
            vec!["/", "<UE>", "|Expr-Div,3,31_|", "<MEP>"],
        ),
        (("<MEP>", "+"), vec![]),
        (("<MEP>", "-"), vec![]),
        (("<MEP>", ">"), vec![]),
        (("<MEP>", "<"), vec![]),
        (("<MEP>", ">="), vec![]),
        (("<MEP>", "<="), vec![]),
        (("<MEP>", "=="), vec![]),
        (("<MEP>", "!="), vec![]),
        (("<MEP>", "&&"), vec![]),
        (("<MEP>", "||"), vec![]),
        (("<MEP>", ";"), vec![]),
        (("<MEP>", ")"), vec![]),
        (("<MEP>", "{"), vec![]),
        (("<UE>", "!"), vec!["!", "<UE>", "|Expr-LogNot,2,_1_|"]),
        (("<UE>", "[Int]"), vec!["<EA>"]),
        (("<UE>", "[Float]"), vec!["<EA>"]),
        (("<UE>", "[String]"), vec!["<EA>"]),
        (("<UE>", "[Char]"), vec!["<EA>"]),
        (("<UE>", "[Bool]"), vec!["<EA>"]),
        (("<UE>", "[Id]"), vec!["<EA>"]),
        (("<UE>", "("), vec!["<EA>"]),
        (("<EA>", "[Int]"), vec!["[Int]", "|Expr-Int,1,__1|"]),
        (("<EA>", "[Float]"), vec!["[Float]", "|Expr-Float,1,__1|"]),
        (("<EA>", "[String]"), vec!["[String]", "|Expr-Str,1,__1|"]),
        (("<EA>", "[Char]"), vec!["[Char]", "|Expr-Char,1,__1|"]),
        (("<EA>", "[Bool]"), vec!["[Bool]", "|Expr-Bool,1,__1|"]),
        (("<EA>", "[Id]"), vec!["[Id]", "|Expr-Id,1,__1|"]),
        (("<EA>", "("), vec!["(", "<OE>", ")", "|Expr-Group,3,_2_|"]),
    ]);

    hash
}
