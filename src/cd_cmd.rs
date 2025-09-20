
use std::env;

pub fn change_directory(path: &str) {
	match env::set_current_dir(path) {
		Ok(()) => (),
		Err(_) => println!("cd: {}: No such file or directory", path),
	}
}