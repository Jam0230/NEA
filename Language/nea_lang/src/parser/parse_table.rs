use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn generate_parse_table() {
    let parse_table_data = fs::read_to_string(
        "/home/jam/Documents/School/ComputerScience/NEA/Language/nea_lang/res/tables/parse_table",
    )
    .expect("AHHHHHHHH but parsing :O");
    let rows = parse_table_data.split(";;").collect::<Vec<&str>>();

    for row in rows {
        let lines = row.split('\n').collect::<Vec<&str>>();
        let left_nt = format!("<{}>", lines[0].split('=').next().unwrap());

        for line_i in 1..lines.len() - 2 {
            let parts = lines[line_i].split("->").collect::<Vec<&str>>();
            let input_expected = parts[0].trim();

            let mut output = parts[1].split(',').collect::<Vec<&str>>();
            output.retain(|x| x != &"");
            let coll_token = format!("|{},{}|", output.len(), left_nt);

            output.push(coll_token.as_str());
            println!("(({:?}, {:?}), vec!{:?}),", left_nt, input_expected, output);
        }
    }
}

pub fn load_parse_table() -> HashMap<(&'static str, &'static str), Vec<&'static str>> {
    // <_> = non-terminal
    // [_] = terminal type
    // |_,_| = collection token (used in parsing to generate ast)
    let hash = HashMap::from([
        (("<P>", "let"), vec!["<SS>", "|1,<P>|"]),
        (("<P>", "[Id]"), vec!["<SS>", "|1,<P>|"]),
        (("<P>", "if"), vec!["<SS>", "|1,<P>|"]),
        (("<P>", "while"), vec!["<SS>", "|1,<P>|"]),
        (("<P>", "$"), vec!["<SS>", "|1,<P>|"]),
        (("<SS>", "let"), vec!["<S>", "<SS>", "|2,<SS>|"]),
        (("<SS>", "[Id]"), vec!["<S>", "<SS>", "|2,<SS>|"]),
        (("<SS>", "if"), vec!["<S>", "<SS>", "|2,<SS>|"]),
        (("<SS>", "while"), vec!["<S>", "<SS>", "|2,<SS>|"]),
        (("<SS>", "}"), vec!["|0,<SS>|"]),
        (("<SS>", "$"), vec!["|0,<SS>|"]),
        (("<S>", "let"), vec!["<DS>", ";", "|2,<S>|"]),
        (("<S>", "[Id]"), vec!["<AS>", ";", "|2,<S>|"]),
        (("<S>", "if"), vec!["<IS>", "|1,<S>|"]),
        (("<S>", "while"), vec!["<WS>", "|1,<S>|"]),
        (
            ("<DS>", "let"),
            vec!["let", "[Type]", "[Id]", "=", "<OE>", "|5,<DS>|"],
        ),
        (
            ("<AS>", "[Id]"),
            vec!["[Id]", "[Assignment]", "<OE>", "|3,<AS>|"],
        ),
        (
            ("<IS>", "if"),
            vec!["if", "<OE>", "{", "<SS>", "}", "<EISP>", "<ES>", "|7,<IS>|"],
        ),
        (("<EISP>", "elif"), vec!["<EIS>", "<EISP>", "|2,<EISP>|"]),
        (("<EISP>", "else"), vec!["|0,<EISP>|"]),
        (("<EISP>", "let"), vec!["|0,<EISP>|"]),
        (("<EISP>", "[Id]"), vec!["|0,<EISP>|"]),
        (("<EISP>", "if"), vec!["|0,<EISP>|"]),
        (("<EISP>", "while"), vec!["|0,<EISP>|"]),
        (("<EISP>", "}"), vec!["|0,<EISP>|"]),
        (("<EISP>", "$"), vec!["|0,<EISP>|"]),
        (
            ("<EIS>", "elif"),
            vec!["elif", "<OE>", "{", "<SS>", "}", "|5,<EIS>|"],
        ),
        (("<ES>", "else"), vec!["else", "{", "<SS>", "}", "|4,<ES>|"]),
        (("<ES>", "let"), vec!["|0,<ES>|"]),
        (("<ES>", "[Id]"), vec!["|0,<ES>|"]),
        (("<ES>", "if"), vec!["|0,<ES>|"]),
        (("<ES>", "while"), vec!["|0,<ES>|"]),
        (("<ES>", "}"), vec!["|0,<ES>|"]),
        (("<ES>", "$"), vec!["|0,<ES>|"]),
        (
            ("<WS>", "while"),
            vec!["while", "(", "<OE>", ")", "{", "<SS>", "}", "|7,<WS>|"],
        ),
        (("<OE>", "!"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Int]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Float]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Str]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Char]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Bool]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "[Id]"), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OE>", "("), vec!["<AE>", "<OEP>", "|2,<OE>|"]),
        (("<OEP>", "||"), vec!["||", "<AE>", "<OEP>", "|3,<OEP>|"]),
        (("<OEP>", ";"), vec!["|0,<OEP>|"]),
        (("<OEP>", ")"), vec!["|0,<OEP>|"]),
        (("<OEP>", "{"), vec!["|0,<OEP>|"]),
        (("<AE>", "!"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Int]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Float]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Str]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Char]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Bool]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "[Id]"), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AE>", "("), vec!["<EE>", "<AEP>", "|2,<AE>|"]),
        (("<AEP>", "&&"), vec!["&&", "<EE>", "<AEP>", "|3,<AEP>|"]),
        (("<AEP>", "||"), vec!["|0,<AEP>|"]),
        (("<AEP>", ";"), vec!["|0,<AEP>|"]),
        (("<AEP>", ")"), vec!["|0,<AEP>|"]),
        (("<AEP>", "{"), vec!["|0,<AEP>|"]),
        (("<EE>", "!"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Int]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Float]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Str]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Char]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Bool]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "[Id]"), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EE>", "("), vec!["<IE>", "<EEP>", "|2,<EE>|"]),
        (("<EEP>", "=="), vec!["==", "<IE>", "<EEP>", "|3,<EEP>|"]),
        (("<EEP>", "!="), vec!["!=", "<IE>", "<EEP>", "|3,<EEP>|"]),
        (("<EEP>", "&&"), vec!["|0,<EEP>|"]),
        (("<EEP>", "||"), vec!["|0,<EEP>|"]),
        (("<EEP>", ";"), vec!["|0,<EEP>|"]),
        (("<EEP>", ")"), vec!["|0,<EEP>|"]),
        (("<EEP>", "{"), vec!["|0,<EEP>|"]),
        (("<IE>", "!"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Int]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Float]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Str]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Char]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Bool]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "[Id]"), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IE>", "("), vec!["<ADE>", "<IEP>", "|2,<IE>|"]),
        (("<IEP>", ">"), vec![">", "<ADE>", "<IEP>", "|3,<IEP>|"]),
        (("<IEP>", "<"), vec!["<", "<ADE>", "<IEP>", "|3,<IEP>|"]),
        (("<IEP>", ">="), vec![">=", "<ADE>", "<IEP>", "|3,<IEP>|"]),
        (("<IEP>", "<="), vec!["<=", "<ADE>", "<IEP>", "|3,<IEP>|"]),
        (("<IEP>", "=="), vec!["|0,<IEP>|"]),
        (("<IEP>", "!="), vec!["|0,<IEP>|"]),
        (("<IEP>", "&&"), vec!["|0,<IEP>|"]),
        (("<IEP>", "||"), vec!["|0,<IEP>|"]),
        (("<IEP>", ";"), vec!["|0,<IEP>|"]),
        (("<IEP>", ")"), vec!["|0,<IEP>|"]),
        (("<IEP>", "{"), vec!["|0,<IEP>|"]),
        (("<ADE>", "!"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Int]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Float]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Str]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Char]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Bool]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "[Id]"), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADE>", "("), vec!["<ME>", "<ADEP>", "|2,<ADE>|"]),
        (("<ADEP>", "+"), vec!["+", "<ME>", "<ADEP>", "|3,<ADEP>|"]),
        (("<ADEP>", "-"), vec!["-", "<ME>", "<ADEP>", "|3,<ADEP>|"]),
        (("<ADEP>", ">"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "<"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", ">="), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "<="), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "=="), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "!="), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "&&"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "||"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", ";"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", ")"), vec!["|0,<ADEP>|"]),
        (("<ADEP>", "{"), vec!["|0,<ADEP>|"]),
        (("<ME>", "!"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Int]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Float]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Str]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Char]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Bool]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "[Id]"), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<ME>", "("), vec!["<UE>", "<MEP>", "|2,<ME>|"]),
        (("<MEP>", "*"), vec!["*", "<UE>", "<MEP>", "|3,<MEP>|"]),
        (("<MEP>", "/"), vec!["/", "<UE>", "<MEP>", "|3,<MEP>|"]),
        (("<MEP>", "+"), vec!["|0,<MEP>|"]),
        (("<MEP>", "-"), vec!["|0,<MEP>|"]),
        (("<MEP>", ">"), vec!["|0,<MEP>|"]),
        (("<MEP>", "<"), vec!["|0,<MEP>|"]),
        (("<MEP>", ">="), vec!["|0,<MEP>|"]),
        (("<MEP>", "<="), vec!["|0,<MEP>|"]),
        (("<MEP>", "=="), vec!["|0,<MEP>|"]),
        (("<MEP>", "!="), vec!["|0,<MEP>|"]),
        (("<MEP>", "&&"), vec!["|0,<MEP>|"]),
        (("<MEP>", "||"), vec!["|0,<MEP>|"]),
        (("<MEP>", ";"), vec!["|0,<MEP>|"]),
        (("<MEP>", ")"), vec!["|0,<MEP>|"]),
        (("<MEP>", "{"), vec!["|0,<MEP>|"]),
        (("<UE>", "!"), vec!["!", "<UE>", "|2,<UE>|"]),
        (("<UE>", "[Int]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "[Float]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "[Str]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "[Char]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "[Bool]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "[Id]"), vec!["<EA>", "|1,<UE>|"]),
        (("<UE>", "("), vec!["<EA>", "|1,<UE>|"]),
        (("<EA>", "[Int]"), vec!["[Int]", "|1,<EA>|"]),
        (("<EA>", "[Float]"), vec!["[Float]", "|1,<EA>|"]),
        (("<EA>", "[Str]"), vec!["[Str]", "|1,<EA>|"]),
        (("<EA>", "[Char]"), vec!["[Char]", "|1,<EA>|"]),
        (("<EA>", "[Bool]"), vec!["[Bool]", "|1,<EA>|"]),
        (("<EA>", "[Id]"), vec!["[Id]", "|1,<EA>|"]),
        (("<EA>", "("), vec!["(", "<OE>", ")", "|3,<EA>|"]),
    ]);

    hash
}
