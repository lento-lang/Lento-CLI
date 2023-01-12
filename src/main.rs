mod conf;
mod args;
mod error;
mod commands;

use std::process::exit;

use args::{lento_args, lento_command};

use error::print_error_usage;
use commands::files::handle_command_files;

fn main() {
    let mut arg_parser = lento_args();
    let args = arg_parser.get_matches_mut();
    match args.subcommand() {
        Some((lento_command::BUILD, _))     => println!("The build command is not yet implemented!"),
        Some((lento_command::COMPILE, _))   => println!("The compile command is not yet implemented!"),
        Some((lento_command::DOC, _))       => println!("The doc command is not yet implemented!"),
        Some((lento_command::EVAL, _))      => println!("The eval command is not yet implemented!"),
        Some((lento_command::FMT, _))       => println!("The fmt command is not yet implemented!"),
        Some((lento_command::LINT, _))      => println!("The lint command is not yet implemented!"),
        Some((lento_command::REPL, _))      => println!("The repl command is not yet implemented!"),
        Some((lento_command::RUN, _))       => println!("The run command is not yet implemented!"),
        Some((lento_command::NEW, _))       => println!("The new command is not yet implemented!"),
        Some((lento_command::TEST, _))      => println!("The test command is not yet implemented!"),
        _ => match args.get_many::<String>("files") {
            Some(raw_files) => handle_command_files(raw_files, &mut arg_parser),
            _ => {
                print_error_usage("No command provided".to_string(), &mut arg_parser);
                println!("{:#?}", args);
                exit(1);
            }
        }
    }
}
