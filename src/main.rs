mod config;
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

	match conf {
		Config::New(link) => new_problem(link),
		Config::Run => run(),
		_ => {}
	}
}
