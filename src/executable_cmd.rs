use std::process::Command;

pub fn run_executable(cmd: &str, args: &[String]) {
	// let mut parts = arg.split_whitespace();
	// let program = parts.next().unwrap();
	Command::new(cmd).args(args).status().ok();
}