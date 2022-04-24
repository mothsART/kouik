#[macro_use]
extern crate clap;
extern crate serde;
#[macro_use]
extern crate diesel;
extern crate procfs;

use std::env;
use std::process::{Command, exit};
use std::io::stdin;

use crate::diesel::Connection;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

pub mod schema;
pub mod models;

mod cli;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
                                crate_name!(), concat_list
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
                                        crate_name!(), program
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
    let matches = cli::build_cli(crate_name!(), crate_version!()).get_matches();
    if let Some(program) = matches.value_of("program") {
        establish_connection();

        let _locale = env::var("LANG").unwrap();
        let lang = _locale.get(0..5);

        let select = format!(
            "SELECT name FROM programs WHERE (locale = \"{}\" OR locale IS NULL) AND editdist3(keyword, \"{}\") < 200;",
            lang.unwrap(),
            program
        );
        let command = Command::new("sqlite3")
                            .arg("karcher.db")
                            .arg(".load ./spellfix")
                            .arg(select)
                            .output()
                            .unwrap()
                            .stdout;
        let stdout = String::from_utf8(command);
        /*
         * Diesel doesn't support SQlite extension :
         * https://github.com/diesel-rs/diesel/discussions/2989
         * => https://github.com/diesel-rs/diesel/pull/2180
        let results = programs.filter(keyword.eq(program))
                              .filter(locale.eq(lang).or(locale.is_null()))
                              .load::<Program>(&connection);
        */

        match stdout {
            Err(_e) => {
                eprintln!(
                    "Nothing was found matching \"{}\". {} can't kill processes.",
                    program, crate_name!()
                );
                exit(1);
            },
            Ok(stdout) => {
                let results: Vec<&str> = stdout.lines().collect();
                if results.is_empty() {
                    eprintln!(
                        "Nothing was found matching \"{}\". {} can't kill processes.",
                        program, crate_name!()
                    );
                    exit(1);
                }
                match procfs::process::all_processes() {
                    Err(_e) => {
                        eprintln!(
                            "Nothing was found matching \"{}\". {} can't kill processes.",
                            program, crate_name!()
                        );
                        exit(1);
                    },
                    Ok(processes) => {
                        let mut kill_list =  Vec::new();
                        for p in results {
                            for prc in &processes {
                                if kill_list.contains(&prc.stat.comm) {
                                    continue;
                                }
                                if prc.stat.comm.contains(&p) {
                                    kill_list.push(prc.stat.comm.to_string());
                                }
                                if kill_list.len() > 9 {
                                    break;
                                }
                            }
                        }

                        if kill_list.is_empty() {
                            eprintln!(
                                "Nothing was found matching \"{}\". {} can't kill processes.",
                                program, crate_name!()
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
