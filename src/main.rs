#[allow(unused_imports)]
use std::io::{self, Write};

const BUILTIN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];
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
            if let Some(cmd) = input.trim().strip_prefix("type") {
                
                let cmd = cmd.trim();
                if BUILTIN_COMMANDS.contains(&cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    println!("{}: not found", cmd);
                }

            }
        }
        else {
            println!("{}: command not found", input.trim());
        }
    }

}
