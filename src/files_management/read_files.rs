use std::fs;
use std::fs::File;
use std::io::ErrorKind;

use crate::args_management::command::Commande;
use crate::errors::print_error::print_error;
use crate::print_data;
use crate::stdin_management::get_input;

const IS_DIR: &'static str = "Is a directory";
const FILE_NOT_FOUND: &'static str = "No such file or directory";
const PERM_DENIED: &'static str = "Permission denied";
const UNEX_ERR: &'static str = "Unexpected error occured";

pub fn read_files(command: &Commande, ret: &mut i32) {
    let mut i = 1;
    for filename in command.path.iter() {
        if filename == "-" {
            get_input::get_stdin_input(command, Some(&mut i));
        } else {
            match fs::metadata(&filename).is_ok() {
                true => {
                    if fs::metadata(&filename).unwrap().is_dir() {
                        print_error(None, IS_DIR);
                        continue;
                    }
                    let file = match File::open(&filename) {
                        Ok(file) => file,
                        Err(error) => match error.kind() {
                            ErrorKind::PermissionDenied => {
                                *ret = 1;
                                print_error(Some(&filename), PERM_DENIED);
                                continue;
                            }
                            _ => {
                                *ret = 1;
                                print_error(Some(&filename), UNEX_ERR);
                                continue;
                            }
                        },
                    };
                    print_data::print_file::print_file_content(&command, &file, &mut i);
                }
                false => {
                    *ret = 1;
                    print_error(Some(&filename), FILE_NOT_FOUND);
                    continue;
                }
            }
        }
    }
}
