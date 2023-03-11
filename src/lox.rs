use std::{fs::read_to_string, io::*};

pub struct Lox {
    //
}

impl Lox {
    pub fn run_file(path: String) {
        let code = read_to_string(path).unwrap();
        println!("{code}");
    }

    pub fn run_prompt() {
        let stdin = stdin();
        let mut stdout = stdout();
        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut line = String::new();
            stdin.read_line(&mut line).expect("Failed to read stdin");
            Lox::run(line);
        }
    }

    pub fn run(code: String) {
        print!("{code}");
    }
}