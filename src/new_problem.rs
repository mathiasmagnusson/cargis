use crate::error::CargisError;
use easy_http_request::DefaultHttpRequest;
use json::object;
use std::{
	fs::{self, File},
	path::Path,
	process::Command,
};
use unzip::Unzipper;

pub fn new_problem(name: &str) -> Result<(), CargisError> {
	// Check if the problem exists
	let response =
		DefaultHttpRequest::get_from_url_str(format!("https://open.kattis.com/problems/{}/file/statement/samples.zip", name))?
			.send()?;

	if response.status_code != 200 {
		return Err(CargisError::ProblemNotFoundError);
	}

	// Create a project with cargo
	let output = match Command::new("cargo")
		.arg("new")
		.arg("--color")
		.arg("always") // TODO: check if stdout is TTY
		.arg("--bin")
		.arg(&name)
		.output()
	{
		Ok(output) => output,
		Err(e) => return Err(CargisError::CargoStartingError(e)),
	};

	if !output.status.success() {
		return Err(CargisError::CargoRunningError(
			output.stderr,
			output.status.code(),
		));
	}

	// Add file in the project dir to keep the link available for submission
	fs::create_dir(Path::new(name).join("cargis"))?;

	fs::write(
		Path::new(name).join("cargis").join("problem.txt"),
		object! { "link" => format!("https://open.kattis.com/problems/{}", name) }.dump(),
	)?;

	// Put the test files in the project
	fs::write(
		Path::new(name).join("cargis").join("tests.zip"),
		response.body,
	)?;

	Unzipper::new(
		File::open(Path::new(name).join("cargis").join("tests.zip"))?,
		Path::new(name).join("cargis").join("tests"),
	)
	.unzip()?;

	fs::remove_file(Path::new(name).join("cargis").join("tests.zip"))?;

	// TODO: add different boilerplate to main file
	Ok(())
}
