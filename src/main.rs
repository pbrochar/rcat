use std::env;
use std::process;

mod files_management;
mod args_management;
mod errors;
mod print_data;
mod stdin_management;

use args_management::validity;
use files_management::read_files::read_files;

pub const PROG_NAME: &'static str = "rcat";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ret: i32 = 0;
    let command = validity::args_parser(&args);
    match command.is_valid {
        true => {
            match command.path.is_empty() {
                true    =>  stdin_management::get_input::get_stdin_input(&command, None),
                false   => read_files(&command, &mut ret),
            }
        },
        false => println!("{}: invalid option -- '{}'", PROG_NAME, command.unknown_option.unwrap())
    }
    process::exit(ret);
}
