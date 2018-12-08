mod config;
mod error;
mod new_problem;
mod run;

use crate::config::Config;
use crate::new_problem::new_problem;
use crate::run::run;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let conf = match Config::new(&args) {
		Ok(conf) => conf,
		Err(()) => Config::help(),
	};

	let result = match conf {
		Config::New(name) => new_problem(name),
		Config::Run => run(),
		_ => { std::process::exit(1) }
		// TODO: Add the rest
	};

	if let Err(e) = result {
		eprintln!("{}", e);
		std::process::exit(1);
	}
}
