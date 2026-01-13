use std::time::SystemTime;
use std::time::SystemTimeError;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use std::process::exit;
use std::process::Command;

use std::io::BufReader;
use std::io::prelude::*;

use std::fs::File;

// checks for: eztest [executable path] [test cases path]
pub fn arg_structure_ok(args: &[String])  {
	if args.len() != 3 {
		println!("Usage: eztest [executable] [test cases path]");
		exit(1);
	}
}

// Tests executing command and quits if it doesn't work
pub fn program_exec_ok(arg: &String) {
	Command::new(arg).output().unwrap_or_else(|e| {
		println!("[ERROR] Unable to execute command '{}' ({:?}).", arg, e);
		exit(1);	
	});
}

// Checks to see if [test cases path] exists
pub fn file_or_explode(arg: &String) -> File {
	match File::open(arg) {
		Ok(file) => file,
		Err(e) => { println!("[ERROR] Unable to open '{}' ({:?})", arg, e); exit(1); }
	}
}

// Maps lines from file into array
pub fn get_from_file(file: File) -> Vec<String> {
	let reader = BufReader::new(file);
	reader.lines()
		.map(|line| line.unwrap())
		.collect()
}

// Splits 'line' into arguments and executes them
pub fn execute(exec: &String, line: &str) {
	let args: Vec<&str> = line.split(' ').collect();
	let output = Command::new(exec)
		.args(args)
		.output()
		.expect("Unable to execute command in 'execute' function");

	println!(">> {}", String::from_utf8_lossy(&output.stdout).trim());
}

// Gets current timestamp for calculating runtime
pub fn get_current_time_dur() -> Result<Duration, SystemTimeError> {
	let time = SystemTime::now().duration_since(UNIX_EPOCH)?;
	Ok(time)
}
