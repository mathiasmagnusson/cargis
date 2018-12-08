use self::CargisError::*;
use easy_http_request::HttpRequestError;
use std::{fmt, io};

#[derive(Debug)]
pub enum CargisError {
	IoError(io::Error),
	HttpError(HttpRequestError),
	ProblemNotFoundError,
	CargoStartingError(io::Error),
	CargoRunningError(Vec<u8>, Option<i32>),
}

impl From<io::Error> for CargisError {
	fn from(err: io::Error) -> Self {
		IoError(err)
	}
}

impl From<HttpRequestError> for CargisError {
	fn from(err: HttpRequestError) -> Self {
		HttpError(err)
	}
}

impl fmt::Display for CargisError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "\x1b[31merror:\x1b[0m ")?;

		match self {
			IoError(err) => fmt::Display::fmt(&err, f),
			HttpError(err) => match err {
				HttpRequestError::UrlParseError(_) => write!(f, "Could not parse the supplied url."),
				HttpRequestError::HttpError(err) => fmt::Display::fmt(&err, f),
				HttpRequestError::HyperError(_) =>  write!(f, "Could not find host \"open.kattis.com\", make sure you're connected to the internet!"),
				HttpRequestError::IOError(err) => fmt::Display::fmt(&err, f),
				HttpRequestError::FromUtf8Error(err) => fmt::Display::fmt(&err, f),
				HttpRequestError::TooLarge => write!(f, "An HttpRequestError::TooLarge error has occured! I have no idea what this means!"),
				HttpRequestError::Other(s) => write!(f, "An HttpRequestError::Other error has occured! The string provided is: {}", s),
			},
			ProblemNotFoundError => write!(f, "It doesn't seem like kattis has that problem. Make sure you spelled it correctly."),
			CargoStartingError(err) => write!(f, "Could not start cargo. Make sure that `cargo` is in your path. Error received:\n{}", &err),
			CargoRunningError(_stderr, code) => {
				write!(f, "There was a problem running cargo. ")?;
				if let Some(code) = code {
					write!(f, "The termination code was: {}\n", code)?;
				}

				// TODO: write out stderr, it's a Vec<u8> and I'm lazy

				Ok(())
			}
		}
	}
}
