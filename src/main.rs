use std::{env, fs};

mod ast;
mod error;
mod lexer;
mod parser;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[1];
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let mut parser = parser::Parser::new(&contents);
    // for _ in 0..14 {
    //     println!("---------------------------------");
    //     dbg!(parser.next(), parser.text());
    //     println!("---------------------------------");
    //     println!();
    // }
    let imp = "fn addasdasd(x: String, y: String) {
        fn hi();
    }";
    println!("{:#?}", parser::Parser::parse(imp));
}
