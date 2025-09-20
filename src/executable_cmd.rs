use std::process::Command;

pub fn run_executable(arg: String) {
	let mut parts = arg.split_whitespace();
	let program = parts.next().unwrap();
	Command::new(program).args(parts).status().ok();
}