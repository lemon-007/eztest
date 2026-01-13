use std::time::Duration;
use std::env;
use std::fs::File;
mod util;

fn main() {
	let args: Vec<String> = env::args().collect();
	util::arg_structure_ok(&args); // checks for 'eztest [arg] [arg]'

	let program_path: &String = &args[1];
	util::program_exec_ok(program_path);

	// '&args[2] is the test case file path'
	let file: File = util::file_or_explode(&args[2]);
	let args_from_file: Vec<String> = util::get_from_file(file);

	println!("Running '{}' against {} test cases", program_path, args_from_file.len());

	// Runs each test case
	for line in args_from_file {
		let stopwatch = util::get_current_time_dur().unwrap_or_else(|e| {
			println!("(Unable to get timestamp #1 {:?})", e);
			Duration::from_secs(0)
		});
		
		println!("{} {}", program_path, line);
		util::execute(program_path, &line);

		let stopwatch_end = util::get_current_time_dur().unwrap_or_else(|e| {
			print!("(Unable to get timestamp #2 {:?}) ", e);
			Duration::from_secs(0)
		});

		let final_time = stopwatch_end - stopwatch; 
		println!("Finished in {:.3}ms\n", (final_time.as_secs_f64() * 1000.0));
	}
}
