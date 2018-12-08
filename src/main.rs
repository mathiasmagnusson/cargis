mod config;
mod error;
mod new_problem;
mod submit;
mod test;

use crate::config::Config;
use crate::new_problem::new_problem;
use crate::submit::submit;
use crate::test::test_problem;
#[macro_use]
extern crate text_io;

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let conf = match Config::new(&args) {
		Ok(conf) => conf,
		Err(()) => Config::help(),
	};

	let result = match conf {
		Config::New(name) => new_problem(name),
		Config::Test => test_problem(),
		Config::Submit => submit(),
	};

	if let Err(e) = result {
		eprintln!("{}", e);
		std::process::exit(1);
	}
}
