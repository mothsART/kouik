use std::process::{Command, exit};
use std::io::stdin;

extern crate procfs;
use clap::{App, Arg};

const VERSION: &'static str = "0.1.0";
const APP_NAME: &'static str = "Karcher";

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
    println!("q) quit");

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
    let matches = build_cli().get_matches();
    if let Some(program) = matches.value_of("program") {
        let mut search_p = Vec::new();
        if program == "navigateur" {
            search_p.push("firefox");
            search_p.push("chromium");
        }
        if search_p.len() == 0 {
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
                for p in &search_p {
                    for prc in &processes {
                        if kill_list.contains(&prc.stat.comm) {
                            continue;
                        }
                        if prc.stat.comm.contains(p) {
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
