#[allow(unused_imports)]
use std::io::{self, Write};

mod cd_cmd;
mod executable_cmd;
mod pwd_cmd;
mod type_cmd;
mod utils;

// cat '/tmp/bar/f   55' '/tmp/bar/f   1' '/tmp/bar/f   34'

fn main() {
    // moving this outside to avoid re-allocating every iteration
    let mut input: String = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        io::stdin().read_line(&mut input).unwrap();

        // let mut parts = input.trim().split_whitespace();
        let parts = utils::parse_args(input.trim());
        let cmd = parts.get(0).unwrap().as_str();
        let args = &parts[1..];

        match cmd {
            "exit" => {
                return;
            }
            "echo" => {
                let echo_text = args.join(" ");
                println!("{}", echo_text.trim());
            }
            "type" => {
                type_cmd::check_type(input.trim());
            }
            "pwd" => {
                let cwd = pwd_cmd::get_pwd();
                println!("{}", cwd.into_os_string().into_string().unwrap());
            }
            "cd" => {
                cd_cmd::change_directory(&args.join(" "));
            }
            _ => {
                if type_cmd::get_executable(cmd).is_some() {
                    executable_cmd::run_executable(cmd, args);
                } else {
                    println!("{}: command not found", input.trim());
                }
            }
        }

        input.clear();
    }
}
