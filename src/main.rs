use clap::{App, Arg};
use std::io::{stdin,stdout,Write};

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
                    
                    /* calcul leveinstein distance pour tous */
                    let proc_with_levensthein_distance = lib::obtain_levensthein_distance(program_name,liste_procs);

                    let value_max_to_be_close : usize = program_name.chars().count()/2 - 1;

                    if value_max_to_be_close <= 0 {
                        println!("Aucun processus ne correspond au nom {:?}", program_name);
                        return;
                    }

                    println!("La valeur maximale est {}", value_max_to_be_close);

                    let mut processus_similar = Vec::<lib::Proc>::new();

                    for processus in proc_with_levensthein_distance {
                        if processus.levensthein_distance <= value_max_to_be_close {
                            println!("Trouvé le programme {:?} car levensthein_distance est de {}", processus.proc.names, processus.levensthein_distance);
                            processus_similar.push(processus.proc);
                        } 
                    }
                    
                    let accept_choice = interact_with_user_ask_if_it_must_kill(program_name, &processus_similar);

                    if let Some(accepted_index) = accept_choice {
                        if let Some(procs) = processus_similar.get(accepted_index) {
                            println!("et tada ! {:?}", procs.names);
                        }
                    }
                }
            },
            Err(_) => {
                println!("flûte une erreur s'est produite !");
            },
        }
    }
}

fn interact_with_user_ask_if_it_must_kill(progname: &str, processus_similar: &Vec<lib::Proc>) -> Option<usize> {

    match processus_similar.len() {
        /* if there are one programme say Yes or No*/
        1 => {
            println!("Un processus au nom similaire à été trouvé pour \"{}\"", progname);
            print!("Voulez vous tuer le processus ? (o/N)\t");
            stdout().flush().expect("Le flush de stdout à échoué");
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if s == "O\n" || s == "o\n" {
                return Some(0);
            }
            return None;
        }
        /* if no programme find, then send error message */
        0 => {
            println!("Aucun processus trouvé pour le nom \"{}\"", progname);
            return None;
        }
        /* if there are several programme, choose */
        _ => {
            println!("Plusieurs processus au nom similaire ont étés trouvés pour \"{}\"", progname);
            return None; // a implémenter
        }
    }
}
