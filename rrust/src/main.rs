mod scanner;
mod token;
mod utils;
use scanner::{
    Scanner
};
use std::env;
use std::io;
use std::io::Write;
use std::process::exit;
fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
    return line.trim().to_string()
}
fn run_file(path: &str) {
    println!("{:?}",path)
}
fn run_prompt() {
    loop {
        let input = prompt("> ");
        if input.as_str() == ".exit" {
            exit(0);
        }
        run(&input);
    }
}
fn run(line :&String){
    println!("{}",line);
    let sc = Scanner {};
    // scanner.scan_tokens() -> tokens -> parser.parse -> statments -> interpreter(statements)
    sc.scan_tokens(line);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // for elem in args.iter() {
    //     println!("{:?}", elem)
    // }
    match args.len() {
        1 => {
            run_prompt()
        }
        2 => {
            run_file(&args[1])
        }
        _ => {}
    }
}
