#[allow(unused_imports)]
use std::io::{self, Write};

mod executable_cmd;
mod type_cmd;

fn main() {
    // moving this outside to avoid re-allocating every iteration
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts.collect::<Vec<&str>>();
        
        match cmd {
            "exit" => {
                return;
            },
            "echo" => {
                let echo_text = args.join(" ");
                println!("{}", echo_text.trim());
            },
            "type" => {
                type_cmd::check_type(input.trim());
            },
            _ => {
                if type_cmd::get_executable(cmd).is_some() {
                    executable_cmd::run_executable(input.trim().to_string());
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
        }

        input.clear();
    }
}
