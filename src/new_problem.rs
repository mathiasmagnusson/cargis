use easy_http_request::DefaultHttpRequest;
use json::object;
use std::{
	fs::{self, File},
	io::{self, Write},
	path::Path,
	process::Command,
};
use unzip::Unzipper;

pub fn new_problem(link: &str) {
	// Get the name of the problem
	let name = link.split('/').next_back().unwrap(); // TODO: error handling

	// Create a project with cargo new --bin {name}
	let output = match Command::new("cargo")
		.arg("new")
		.arg("--color")
		.arg("always") // TODO: check if stdout is TTY
		.arg("--bin")
		.arg(&name)
		.output()
	{
		Ok(output) => output,
		Err(e) => {
			eprintln!("Could not start cargo. Error received:\n{}\nMake sure that `cargo` is in your path", e);
			std::process::exit(1);
		}
	};

	if !output.status.success() {
		eprintln!("Cargo did not run successfully:");

		io::stderr().write(&output.stderr).unwrap();

		std::process::exit(output.status.code().unwrap_or(1));
	}

	// Add file in the project dir to keep {link} available for submission
	fs::create_dir(Path::new(name).join("cargis")).unwrap(); // TODO: error handling

	fs::write(
		Path::new(name).join("cargis").join("problem.txt"),
		object! { "link" => link }.dump(),
	)
	.unwrap(); // TODO: error handling

	// Set up all tests available for the problem
	let response =
		DefaultHttpRequest::get_from_url_str(format!("{}/file/statement/samples.zip", link))
			.unwrap()
			.send()
			.unwrap();

	if response.status_code != 200 {
		eprintln!("Couldnt reach {}", link);
		std::process::exit(1);
		// Don't create the cargo project if the problem wasn't found
	}

	fs::write(
		Path::new(name).join("cargis").join("tests.zip"),
		response.body,
	)
	.unwrap();

	Unzipper::new(
		File::open(Path::new(name).join("cargis").join("tests.zip")).unwrap(), // TODO: error handling
		Path::new(name).join("cargis").join("tests"),
	)
	.unzip()
	.unwrap(); // TODO: error handling

	fs::remove_file(Path::new(name).join("cargis").join("tests.zip")).unwrap();

	// TODO: add different boilerplate to main file
}
