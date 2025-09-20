use std::path::Path;
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;

const BUILTIN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];

pub fn check_type(command: &str) {
	if let Some(cmd) = command.trim().strip_prefix("type") {
		let cmd = cmd.trim();
		if BUILTIN_COMMANDS.contains(&cmd) {
			println!("{} is a shell builtin", cmd);
		} else {
			let path = env::var("PATH").expect("PATH must be set");
			for path_elem in path.split(":") {
				let file_path_str = &format!("{}/{}", path_elem, cmd);
				let file_path = Path::new(file_path_str);
				if file_path.exists() {
					if let Ok(metadata) = fs::metadata(file_path) {
						let permissions = metadata.permissions();
						if permissions.mode() & 0o111 != 0 {
							println!("{} is {}", cmd, file_path_str);
							return;
						}
					}
				}
			}
			println!("{}: not found", cmd);
		}
	}
}