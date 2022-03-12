use std::env;

use std::io::BufWriter;
use std::io::prelude::*;

fn main() {
	let stdout = std::io::stdout();
	let mut writer = BufWriter::new(stdout);

	let user = env::var("USER").unwrap();

	if user == "root" {
		writer.write_all(format!("{} {}\n", "", user).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else {
		writer.write_all(format!("{} {}\n", "", user).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
}
