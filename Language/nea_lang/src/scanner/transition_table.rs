use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn generate_trans_table() {
    // just to create list for trans_table loading (might change to just load from file later)
    let trans_data = fs::read_to_string(
        "/home/jam/Documents/School/ComputerScience/NEA/Language/nea_lang/res/tables/trans_table",
    )
    .expect("AHHHHHHHHHHHHH");

    let lines = trans_data.split('\n');

    let mut current = 0;
    let mut values: Vec<(&str, u32)> = Vec::new();

    for line in lines {
        if line.len() == 0 {
            println!("({}, vec!{:?}),", current, values);
            break;
        }
        let temp: Vec<&str> = line.split(':').collect();
        if temp[0].parse::<i32>().expect("AHHHHH") == current {
            values.push((temp[1], temp[2].parse::<u32>().expect("AHHHH")));
        } else {
            println!("({}, vec!{:?}),", current, values);
            values = Vec::new();
            values.push((temp[1], temp[2].parse::<u32>().expect("AHHHH")));

            current = temp[0].parse::<i32>().expect("AHHH");
        }
    }
}

pub fn load_trans_table() -> (
    HashMap<u32, Vec<(&'static str, u32)>>,
    Vec<(u32, &'static str)>,
) {
    // Im really sorry for this future me ;-;
    let hash: HashMap<u32, Vec<(&'static str, u32)>> = HashMap::from([
        (
            0,
            vec![
                ("i", 1),
                ("e", 6),
                ("m", 12),
                ("c", 18),
                ("f", 32),
                ("w", 35),
                ("b", 38),
                ("l", 42),
                ("d", 44),
                ("r", 45),
                ("s", 50),
                ("p", 54),
                ("[0-9]", 61),
                ("\"", 64),
                ("'", 66),
                ("t", 80),
                ("\\s", 69),
                ("\\/", 70),
                ("[(){}\\[\\]:;.,]", 72),
                ("[!]", 76),
                ("&", 78),
                ("\\|", 79),
                ("[ag-hj-kn-oqt-vx-z_]", 86),
                ("[<>]", 87),
                ("[+\\-*%]", 88),
                ("=", 89),
            ],
        ),
        (
            1,
            vec![("f", 34), ("n", 30), ("m", 2), ("[a-eg-lo-z_0-9]", 86)],
        ),
        (2, vec![("p", 3), ("[a-oq-z_0-9]", 86)]),
        (3, vec![("o", 4), ("[a-np-z_0-9]", 86)]),
        (4, vec![("r", 5), ("[a-qs-z_0-9]", 86)]),
        (5, vec![("t", 34), ("[a-su-z_0-9]", 86)]),
        (6, vec![("l", 7), ("n", 10), ("[a-kmo-z_0-9]", 86)]),
        (7, vec![("s", 9), ("i", 8), ("[a-hj-rt-z_0-9]", 86)]),
        (8, vec![("f", 34), ("[a-eg-z_0-9]", 86)]),
        (9, vec![("e", 34), ("[a-df-z_0-9]", 86)]),
        (10, vec![("u", 11), ("[a-tv-z_0-9]", 86)]),
        (11, vec![("m", 34), ("[a-ln-z_0-9]", 86)]),
        (12, vec![("a", 13), ("[a-ln-z_0-9]", 86)]),
        (13, vec![("t", 14), ("[a-su-z_0-9]", 86)]),
        (14, vec![("c", 15), ("[a-bd-z_0-9]", 86)]),
        (15, vec![("h", 34), ("[a-gi-z_0-9]", 86)]),
        (16, vec![("a", 17), ("[b-z_0-9]", 86)]),
        (17, vec![("r", 31), ("[a-qs-z_0-9]", 86)]),
        (
            18,
            vec![
                ("h", 16),
                ("a", 19),
                ("o", 20),
                ("l", 25),
                ("[b-gi-kmnp-z_0-9]", 86),
            ],
        ),
        (19, vec![("s", 9), ("[a-rt-z_0-9]", 86)]),
        (20, vec![("n", 21), ("[a-mo-z_0-9]", 86)]),
        (21, vec![("t", 22), ("[a-su-z_0-9]", 86)]),
        (22, vec![("i", 23), ("[a-hj-z_0-9]", 86)]),
        (23, vec![("n", 24), ("[a-mo-z_0-9]", 86)]),
        (24, vec![("u", 9), ("[a-tv-z_0-9]", 86)]),
        (25, vec![("a", 26), ("[b-z_0-9]", 86)]),
        (26, vec![("s", 27), ("[a-rt-z_0-9]", 86)]),
        (27, vec![("s", 34), ("[a-rt-z_0-9]", 86)]),
        (28, vec![("o", 29), ("[a-np-z_0-9]", 86)]),
        (29, vec![("a", 30), ("[b-z_0-9]", 86)]),
        (30, vec![("t", 31), ("[a-su-z_0-9]", 86)]),
        (
            32,
            vec![("l", 28), ("o", 33), ("a", 84), ("[b-kmnp-z_0-9]", 86)],
        ),
        (33, vec![("r", 34), ("[a-ps-z_0-9]", 86)]),
        (35, vec![("h", 36), ("[a-gi-z_0-9]", 86)]),
        (36, vec![("i", 37), ("[a-hj-z_0-9]", 86)]),
        (37, vec![("l", 9), ("[a-km-z_0-9]", 86)]),
        (38, vec![("r", 39), ("o", 59), ("[a-npqs-z_0-9]", 86)]),
        (39, vec![("e", 40), ("[a-df-z_0-9]", 86)]),
        (40, vec![("a", 41), ("[b-z_0-9]", 86)]),
        (41, vec![("k", 34), ("[a-jl-z_0-9]", 86)]),
        (42, vec![("e", 43), ("[a-df-z_0-9]", 86)]),
        (43, vec![("t", 34), ("[a-su-z_0-9]", 86)]),
        (44, vec![("e", 8), ("[a-df-z_0-9]", 86)]),
        (45, vec![("e", 46), ("[a-df-z_0-9]", 86)]),
        (46, vec![("t", 47), ("[a-su-z_0-9]", 86)]),
        (47, vec![("u", 48), ("[a-tv-z_0-9]", 86)]),
        (48, vec![("r", 49), ("[a-qs-z_0-9]", 86)]),
        (49, vec![("n", 34), ("[a-mo-z_0-9]", 86)]),
        (50, vec![("t", 51), ("[a-su-z_0-9]", 86)]),
        (51, vec![("r", 52), ("[a-qs-z_0-9]", 86)]),
        (52, vec![("u", 53), ("[a-tv-z_0-9]", 86)]),
        (53, vec![("c", 43), ("[a-bd-z_0-9]", 86)]),
        (54, vec![("r", 55), ("u", 57), ("[a-qstv-z_0-9]", 86)]),
        (55, vec![("i", 56), ("[a-hj-z_0-9]", 86)]),
        (56, vec![("v", 34), ("[a-uw-z_0-9]", 86)]),
        (57, vec![("b", 34), ("[ac-z_0-9]", 86)]),
        (59, vec![("o", 60), ("[a-np-z_0-9]", 86)]),
        (60, vec![("l", 31), ("[a-km-z_0-9]", 86)]),
        (61, vec![("[0-9]", 61), ("\\.", 62)]),
        (62, vec![("[0-9]", 63)]),
        (63, vec![("[0-9]", 63)]),
        (64, vec![("[^\"]", 64), ("\"", 65)]),
        (66, vec![("[^']", 67)]),
        (67, vec![("'", 68)]),
        (80, vec![("r", 81), ("[a-qs-z_0-9]", 86)]),
        (81, vec![("u", 82), ("[a-tv-z_0-9]", 86)]),
        (82, vec![("e", 83), ("[a-df-z_0-9]", 86)]),
        (84, vec![("l", 85), ("[a-km-z_0-9]", 86)]),
        (85, vec![("s", 82), ("[a-rt-z_0-9]", 86)]),
        (69, vec![("\\s", 69)]),
        (70, vec![("\\/", 71), ("\\*", 73)]),
        (71, vec![(".", 71)]),
        (73, vec![("[^*]", 73), ("\\*", 74)]),
        (74, vec![("[^/]", 73), ("\\/", 75)]),
        (76, vec![("=", 77)]),
        (78, vec![("&", 77)]),
        (79, vec![("\\|", 77)]),
        (87, vec![("=", 77)]),
        (88, vec![("=", 90)]),
        (89, vec![("=", 77)]),
    ]);
    let excepting_states = vec![
        (1, "Id"),
        (2, "Id"),
        (3, "Id"),
        (4, "Id"),
        (5, "Id"),
        (6, "Id"),
        (7, "Id"),
        (8, "Id"),
        (9, "Id"),
        (10, "Id"),
        (11, "Id"),
        (12, "Id"),
        (13, "Id"),
        (14, "Id"),
        (15, "Id"),
        (16, "Id"),
        (17, "Id"),
        (18, "Id"),
        (19, "Id"),
        (20, "Id"),
        (21, "Id"),
        (22, "Id"),
        (23, "Id"),
        (24, "Id"),
        (25, "Id"),
        (26, "Id"),
        (27, "Id"),
        (28, "Id"),
        (29, "Id"),
        (30, "Id"),
        (32, "Id"),
        (33, "Id"),
        (35, "Id"),
        (36, "Id"),
        (37, "Id"),
        (38, "Id"),
        (39, "Id"),
        (40, "Id"),
        (41, "Id"),
        (42, "Id"),
        (43, "Id"),
        (44, "Id"),
        (45, "Id"),
        (46, "Id"),
        (47, "Id"),
        (48, "Id"),
        (49, "Id"),
        (50, "Id"),
        (51, "Id"),
        (53, "Id"),
        (54, "Id"),
        (55, "Id"),
        (56, "Id"),
        (57, "Id"),
        (31, "Type"),
        (34, "Keyword"),
        (52, "Type"),
        (61, "Int"),
        (63, "Float"),
        (65, "String"),
        (68, "Char"),
        (83, "Bool"),
        (69, "WhiteSpace"),
        (70, "Operator"),
        (71, "Comment"),
        (72, "Delim"),
        (75, "Comment"),
        (76, "Operator"),
        (77, "Comparison"),
        (80, "Id"),
        (81, "Id"),
        (82, "Id"),
        (84, "Id"),
        (85, "Id"),
        (86, "Id"),
        (87, "Comparison"),
        (88, "Operator"),
        (89, "Assignment"),
        (90, "Assignment"),
    ];
    (hash, excepting_states)
}