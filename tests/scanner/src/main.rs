fn remove_white_space(program: &str) -> String {
    program.chars().filter(|x| !x.is_whitespace()).collect() // filers out characters that are not whitespace
}

fn main() {
    println!("{}", remove_white_space("Hello There!\n:3"));
}
