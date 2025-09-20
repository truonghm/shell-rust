use std::env;

pub fn get_pwd() -> String {
	env::current_dir().unwrap().into_os_string().into_string().unwrap()
}