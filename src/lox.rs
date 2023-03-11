use std::fs::read_to_string;

pub struct Lox {
    //
}

impl Lox {
    pub fn run_file(path: String) {
        let code = read_to_string(path).unwrap();
        println!("{code}");
    }

    pub fn run_prompt() {

    }
}