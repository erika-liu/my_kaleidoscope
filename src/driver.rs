use crate::lexer::tokenize;
use std::io;
use std::io::Write;

pub fn main_loop() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut input = String::new();
    loop {
        print!("> ");
        stdout.flush().unwrap();
        input.clear();
        stdin.read_line(&mut input).ok().expect("fail to read line");
        if input.as_str().trim() == "q" {
            break;
        }

        let tokens = tokenize(input.as_str());
        println!("{:?}", tokens);
    }
}
