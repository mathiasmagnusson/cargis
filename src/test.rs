use crate::error::CargisError;
use crate::submit::submit;
use std::{
	fs,
	io::{self, Write},
	path::Path,
	process::{Command, Stdio},
};
use text_io::{read, try_read, try_scan};

pub fn test_problem() -> Result<(), CargisError> {
	// Build the project
	println!("Building project...");
	let output = match Command::new("cargo").arg("build").output() {
		Ok(output) => output,
		Err(e) => return Err(CargisError::CargoStartingError(e)),
	};

	if !output.status.success() {
		return Err(CargisError::CargoRunningError(
			output.stderr,
			output.status.code(),
		));
	}

	println!("Project built successfully!");

	let mut passed_all = true;
	for test in fs::read_dir(Path::new("cargis").join("tests"))? {
		let test = test?;
		let path = test.path();
		let file_name = test.file_name().into_string().unwrap();
		let name = file_name.split('.').nth(0).unwrap().to_owned();

		// Make sure that the file contains input
		if file_name.split('.').last().unwrap() != "in" {
			continue;
		}

		let input = fs::read(path)?;
		let answer = fs::read(
			Path::new("cargis")
				.join("tests")
				.join(&(name.clone() + ".ans")),
		)?;

		println!("Running test {}", name);

		// TODO: Get name of executable
		let mut child = Command::new("target/debug/twostones.exe")
			.stdout(Stdio::piped())
			.stdin(Stdio::piped())
			.spawn()?;

		{
			let stdin = child
				.stdin
				.as_mut()
				.expect("Could not write to program's stdin");

			stdin.write(&input)?;
			stdin.flush()?;
		}

		let output = child
			.wait_with_output()
			.expect("Could not read from program's stdout");

		if output.stdout == answer {
			println!("Test {} succeded!", name);
		} else {
			println!("Test {} failed!", name);
			println!("The input was:");
			io::stdout().write(&input)?;
			println!("\nThe correct answer was: ");
			io::stdout().write(&answer)?;
			println!("\nThe program supplied the answer: ");
			io::stdout().write(&output.stdout)?;
			passed_all = false;
		}
	}

	if passed_all {
		print!("All tests succeeded. Do you want to submit the program to kattis? (Y/n) ");
		io::stdout().flush()?;
		let s: String = read!("{}\r\n"); // Problem on other places than windows?

		match s.as_ref() {
			"y" | "Y" | "1" | "yes" | "Yes" | "YES" | "" => {
				submit()?;
			}
			_ => {}
		}
	}

	Ok(())
}
