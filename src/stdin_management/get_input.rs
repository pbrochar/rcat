use crate::args_management::command::Commande;
use crate::print_data::print_file::print_content;
use std::{io, io::prelude::*};

pub fn get_stdin_input(command: &Commande, index: Option<&mut i32>) {
	let line_index;
	let mut i = 1;
	match index {
		Some(x) => line_index = x,
		None => line_index = &mut i,
	}
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		match line {
			Ok(line) => print_content(command, &line.trim_end().to_string(), line_index),
			Err(_error) => {
				println!("rcat: an error occured");
				break;
			}
		}
	}
}
