use crate::PROG_NAME;

pub fn print_error(filename: Option<&str>, error_msg: &str) {
	match filename {
			Some(filename) => eprintln!("{}: {}: {}", PROG_NAME, &filename, error_msg),
			None => eprintln!("{}: {}", PROG_NAME, error_msg),
	}
}