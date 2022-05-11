use super::command;

pub fn args_parser(args: &Vec<String>) -> command::Commande {
    let mut command = command::Commande {
        path: Vec::new(),
        n_option: false,
        e_option: false,
        is_valid: true,
        unknown_option: None,
    };
    parse_command(&mut command, args);
    command
}

fn parse_command(command: &mut command::Commande, args: &Vec<String>) {
    for arg in &args[1..] {
        match &arg[..1] {
            "-" => 
                {  
                    if arg.len() == 1 {
                        command.add_path(&arg);
                    } else {
                        set_letter_option(&arg, command);
                    }
                },
            _   => command.add_path(&arg),
        }
        
    }
}

fn set_letter_option(arg: &String, command: &mut command::Commande) {
    for letter in arg[1..].chars() {
        match letter {
            'n' => command.set_n_option(true),
            'e' => command.set_e_option(true),
            _ => {
                command.set_validity(false, letter);
                return;
            }
        }
    }
}
