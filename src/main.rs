extern crate serde;
#[macro_use]
extern crate diesel;
extern crate procfs;

use std::env;
use std::process::{Command, exit};
use std::io::stdin;

use clap::{App, Arg};
use diesel::prelude::*;
use crate::diesel::Connection;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub mod schema;
pub mod models;


use crate::models::Program;

const VERSION: &'static str = "0.1.0";
const APP_NAME: &'static str = "Karcher";

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn build_cli() -> App<'static, 'static> {
    App::new(APP_NAME)
    .bin_name("karcher")
    .version(VERSION)
    .author("Ferry Jérémie ferryjeremie@free.fr")
    .about("kill program")
    .arg(Arg::with_name("program").required(false).index(1))
}

fn prompt(title: &str, kill_list: &Vec<String>) {
    println!("{}", title);
    for (i, p) in kill_list.iter().enumerate() {
        println!("{}) {}", i + 1, p);
    }
    if kill_list.len() > 1 {
        println!("a) all");
    }
    println!("\nq) quit");

    let mut input = String::new();
    let read_input = stdin().read_line(&mut input);
    match read_input {
        Err(_e) => {
            prompt("Wrong input \"a\". Retry :", kill_list)
        },
        Ok(_r) => {
            match input.trim() {
                "q" => {
                    exit(1);
                },
                "a" => {
                    if kill_list.len() == 1 {
                        prompt("Wrong input \"a\". Retry :", kill_list)
                    }
                    let concat_list = kill_list.join(" ");
                    let killall_cmd = Command::new("killall")
                                              .args(kill_list)
                                              .output();
                    match killall_cmd {
                        Ok(_v) => {
                            println!(
                                "\"{}\" has been killed",
                                concat_list
                            );
                        },
                        Err(_e) => {
                            eprintln!(
                                "{} did'nt succeed in killing {}",
                                APP_NAME, concat_list
                            );
                        }
                    }
                },
                input => {
                    match input.parse::<usize>() {
                        Ok(value) if value > 0 && value <= kill_list.len() => {
                            let program = kill_list.get(value - 1).unwrap();
                            let killall_cmd = Command::new("killall")
                                                      .arg(program)
                                                      .output();
                            match killall_cmd {
                                Ok(_v) => {
                                    println!("\"{}\" has been killed", program);
                                },
                                Err(_e) => {
                                    eprintln!(
                                        "{} did'nt succeed in killing {}",
                                        APP_NAME, program
                                    );
                                }
                            }
                        }
                        Ok(input) => {
                            prompt(
                                &format!("Wrong input \"{}\". Retry :", input),
                                kill_list
                            )
                        },
                        Err(_e) => {
                            prompt(
                                &format!("Wrong input \"{}\". Retry :", input),
                                kill_list
                            )
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    use self::schema::programs::dsl::*;

    let matches = build_cli().get_matches();
    if let Some(program) = matches.value_of("program") {
        let connection = establish_connection();

        let results = programs.filter(keyword.eq(keyword))
            .load::<Program>(&connection);

        match results {
            Err(_e) => {
                eprintln!(
                    "Nothing was found matching \"{}\". {} can't kill processes.",
                    program, APP_NAME
                );
                exit(1);
            },
            Ok(results) => {
                if results.len() == 0 {
                    eprintln!(
                        "Nothing was found matching \"{}\". {} can't kill processes.",
                        program, APP_NAME
                    );
                    exit(1);
                }
                match procfs::process::all_processes() {
                    Err(_e) => {
                        eprintln!(
                            "Nothing was found matching \"{}\". {} can't kill processes.",
                            program, APP_NAME
                        );
                        exit(1);
                    },
                    Ok(processes) => {
                        let mut kill_list =  Vec::new();
                        for p in &results {
                            for prc in &processes {
                                if kill_list.contains(&prc.stat.comm) {
                                    continue;
                                }
                                if prc.stat.comm.contains(&p.name) {
                                    kill_list.push(prc.stat.comm.to_string());
                                }
                                if kill_list.len() > 9 {
                                    break;
                                }
                            }
                        }

                        if kill_list.len() == 0 {
                            eprintln!(
                                "Nothing was found matching \"{}\". {} can't kill processes.",
                                program, APP_NAME
                            );
                            exit(1);
                        }

                        let mut title = "Choose application to kill :";
                        if kill_list.len() > 1 {
                            title = "Choose applications to kill :";
                        }

                        prompt(title, &kill_list);
                    }
                }
            }
        }
    }
}
