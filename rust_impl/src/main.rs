mod scanner;
mod token;
mod utils;
use scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");
    return line.trim().to_string();
}
fn run_file(path: &str) {
    // 创建指向所需的文件的路径
    let path = Path::new(path);
    let display = path.display();

    // 以只读方式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // 读取文件内容到一个字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    run(s);
    println!("{:?}", path)
}
fn run_prompt() {
    loop {
        let input = prompt("> ");
        if input.as_str() == ".exit" {
            exit(0);
        }
        run(input);
    }
}
fn run(input: String) {
    println!("{}", input);
    let mut sc = Scanner::new(input);
    // scanner.scan_tokens() -> tokens -> parser.parse -> statments -> interpreter(statements)
    let tokens = sc.scan_tokens();
    println!("tokens {:?}\n", tokens.len());
}
fn main() {
    let args: Vec<String> = env::args().collect();
    // for elem in args.iter() {
    //     println!("{:?}", elem)
    // }
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {}
    }
}
