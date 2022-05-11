use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::args_management::command::Commande;

pub fn print_file_content(command: &Commande, file: &File, i: &mut i32) {
	for line in BufReader::new(file).lines() {
		if let Ok(this_line) = line {
			print_content(command, &this_line, i);
		}
	}
}

pub fn print_content(command: &Commande, line: &String, i: &mut i32) {
	if command.n_option {
		print!("     {}  ", i);
		*i += 1;
	}
	if command.e_option {
		println!("{}{}", line, "$");
	} else {
		println!("{}", line);
	}
}
