use std::env;
use std::path::PathBuf;

pub fn get_pwd() -> PathBuf {
	env::current_dir().unwrap()
}