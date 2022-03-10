extern crate chrono;

use chrono::offset::*;

use std::io::prelude::*;
use std::io::BufWriter;

pub fn get_date() -> String {
	let get_date = Local::now().date().to_string();
	let rep_date = get_date.replace("-", ".");
	let vec_date = Vec::from(rep_date);
	let str_date = String::from_utf8_lossy(&vec_date[0..10]);
	let ret_date = str_date.to_string();

	return ret_date;
}

fn main() {
	let stdout = std::io::stdout();
	let mut writer = BufWriter::new(stdout);

	writer.write_all(format!("{} {}\n", "", get_date()).as_bytes()).unwrap();
	writer.flush().unwrap();
}
