use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

pub fn capacity() -> i32 {
	match File::open("/sys/class/power_supply/BAT0/capacity") {
		Ok(capacity) => {
			let mut reader = BufReader::new(capacity);
			let mut buffer = String::new();

			match reader.read_line(&mut buffer) {
				Ok(_) => {
					return buffer.trim().parse::<i32>().unwrap();
				}

				Err(e) => {
					println!("{}", e);
					std::process::exit(0);
				}
			}
		}

		Err(e) => {
			println!("{}", e);
			std::process::exit(0);
		}
	}
}

pub fn status() -> String {
	match File::open("/sys/class/power_supply/BAT0/status") {
		Ok(status) => {
			let mut reader = BufReader::new(status);
			let mut buffer = String::new();

			match reader.read_line(&mut buffer) {
				Ok(_) => {
					return buffer.trim().to_string();
				}

				Err(e) => {
					println!("{}", e);
					std::process::exit(0);
				}
			}
		}

		Err(e) => {
			println!("{}", e);
			std::process::exit(0);
		}
	}
}

fn main() {
	let status = status();
	let capacity = capacity();

	let stdout = std::io::stdout();
	let mut writer = BufWriter::new(stdout);

	if capacity <= 20 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			if capacity <= 5 {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
			else if capacity <= 10 {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
			else {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
		}
	}
	else if capacity <= 30 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
	}
	else if capacity <= 40 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
	}
	else if capacity <= 60 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			if capacity <= 50 {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
			else {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
		}
	}
	else if capacity <= 80 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			if capacity <= 70 {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
			else {
				writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
			}
		}
	}
	else if capacity <= 90 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
	}
	else if capacity <= 100 {
		if status != String::from("Discharging") {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
		else {
			writer.write_all(format!("{} {}%\n", "", capacity).as_bytes()).unwrap();
		}
	}
}
