use std::process::Command;
use clap::{App, Arg};

pub mod lib;

const VERSION: &'static str = "0.1.0";
const APP_NAME: &'static str = "Kouik";

fn build_cli() -> App<'static, 'static> {
    App::new(APP_NAME)
    .bin_name("kouik")
    .version(VERSION)
    .author("Ferry Jérémie ferryjeremie@free.fr")
    .about("kill program")
    .arg(Arg::with_name("program").required(false).index(1).help("le nom du programme à tuer"))    
}

fn main() {
    let matches = build_cli().get_matches();
    if let Some(program_name) = matches.value_of("program") {
        /* find exact programme */
        match lib::find_program(program_name) {
            Ok(option) => {
                match option {
                    Some(liste_pid) => {
                        println!("{:?}",liste_pid);
                        /* kill them */
                        Command::new("kill")
                            .arg("-9")
                            .arg(liste_pid[0].to_string())
                            .output()
                            .expect("Failed to execute command");
                    },
                    None => println!("Aucun programme ne correspond à {}", program_name),
                }
            },
            Err(_) => {
                println!("flûte une erreur s'est produite !");
            },
        }
        /* find near programme */
            /* if there are one programme say Yes or No*/
            /* if there are several programme, choose */
        /* if no programme find, then send error message */
    } else {
        println!("show kouik help");
    }
}