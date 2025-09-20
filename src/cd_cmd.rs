
use std::env;

pub fn change_directory(path: &str) {
	let path_expanded: String;
	if path == "~" || path.starts_with("~/") {
		let home = env::var("HOME").unwrap_or("/".to_string());
		path_expanded = path.replacen("~", &home, 1);
	} else {
		path_expanded = path.to_string()
	}
	match env::set_current_dir(path_expanded) {
		Ok(()) => (),
		Err(_) => println!("cd: {}: No such file or directory", path),
	}
}