use clap::{App, Arg};
use nix::unistd::Pid;
use nix::sys::signal::{kill,Signal};

pub mod lib;

const VERSION: &'static str = "0.2.2";
const APP_NAME: &'static str = "Kouik";

fn build_cli() -> App<'static, 'static> {
    App::new(APP_NAME)
    .bin_name("kouik")
    .version(VERSION)
    .author("Ferry Jérémie ferryjeremie@free.fr")
    .author("Gabriel Theuws gaby.theuws@gmail.com")
    .about("kill program")
    .arg(Arg::with_name("program").required(true).index(1).help("le nom du programme à tuer"))    
}

fn main() {
    let matches = build_cli().get_matches();
    if let Some(program_name) = matches.value_of("program") {
        /* find exact programme */
        let procs : Result<Vec<lib::Proc>,std::io::Error> = lib::get_procs();
        match procs {
            Ok(liste_procs) => {
                for process in liste_procs {
                    for name in process.names {
                        if name == program_name.to_string() {
                            match kill(Pid::from_raw(process.pid),Signal::SIGTERM) {
                                Ok(_) => {},
                                Err(e) => println!("kill send : {:?}",e),
                            }
                        }
                    }
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
    }
}
