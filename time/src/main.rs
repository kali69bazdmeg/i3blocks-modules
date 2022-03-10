extern crate chrono;

use chrono::offset::*;

use std::io::prelude::*;
use std::io::BufWriter;

pub fn get_time() -> String {
	let get_date = Local::now().time().to_string();
	let vec_date = Vec::from(get_date);
	let str_date = String::from_utf8_lossy(&vec_date[0..8]);
	let ret_date = str_date.to_string();

	return ret_date;
}

fn main() {
	let time   = get_time();
	let hour   = time.split(":").nth(0).unwrap();
	let minute = time.split(":").nth(1).unwrap();
	let second = time.split(":").nth(2).unwrap();

	let stdout = std::io::stdout();
	let mut writer = BufWriter::new(stdout);

	if hour == "01" || hour == "13" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "02" || hour == "14" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "03" || hour == "15" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "04" || hour == "16" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "05" || hour == "17" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "06" || hour == "18" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "07" || hour == "19" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "08" || hour == "20" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "09" || hour == "21" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "10" || hour == "22" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "11" || hour == "23" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
	else if hour == "12" || hour == "00" {
		writer.write_all(format!("{} {}:{}:{}\n", "", hour, minute, second).as_bytes()).unwrap();
		writer.flush().unwrap();
	}
}
