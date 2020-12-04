use std::process::Command;

fn main() {
	Command::new("bash").args(&["./inject.sh"]).status().unwrap();
	println!("cargo:rerun-if-changed=src/*");
}
