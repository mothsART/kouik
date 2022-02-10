use clap::{App, Arg};

pub mod lib;
pub mod kill;

const VERSION: &'static str = "0.2.3";
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
        
        let procs : Result<Vec<lib::Proc>,std::io::Error> = lib::get_procs();
        
        match procs {
            Ok(liste_procs) => {
                /* find exact programme */
                let nb_killed : u32 = kill::kill_proc_by_name(program_name,&liste_procs);
                if nb_killed == 0 {
                    println!("Aucun processus ne correspond au nom {:?}", program_name);
                    /* calcul leveinstein distance pour tous */
                    let proc_with_levensthein_distance = lib::obtain_levenstein_distance(program_name,liste_procs);
                    
                    for processus in proc_with_levensthein_distance {
                        println!("voilà {:?}", processus.levensthein_distance);
                    }
                        /* if there are one programme say Yes or No*/
                        /* if there are several programme, choose */
                        /* if no programme find, then send error message */
                }
            },
            Err(_) => {
                println!("flûte une erreur s'est produite !");
            },
        }
    }
}
