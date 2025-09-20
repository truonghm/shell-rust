#[allow(unused_imports)]
use std::io::{self, Write};

mod type_cmd;

fn main() {
    while true {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();

        // receive new input after this line
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit 0" {
            return;
        } else if input.trim().starts_with("echo") {
            if let Some(echo_text) = input.trim().strip_prefix("echo") {
                println!("{}", echo_text.trim());
            }
        } else if input.trim().starts_with("type") {
            type_cmd::check_type(input.trim());
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
