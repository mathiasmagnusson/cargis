pub enum Config<'a> {
	New(&'a str),
	Run,
	Test,
	Submit,
}

impl Config<'_> {
	pub fn new(args: &[String]) -> Result<Config, ()> {
		match args[1].as_ref() {
			"new" | "create" => {
				if args.len() < 3 {
					Err(())
				} else {
					Ok(Config::New(&args[2]))
				}
			}
			"run" => Ok(Config::Run),
			"test" | "judge" => Ok(Config::Test),
			"submit" => Ok(Config::Submit),
			_ => Err(()),
		}
	}

	pub fn help() -> ! {
		println!("Insert help message here");
		std::process::exit(1);
	}
}
