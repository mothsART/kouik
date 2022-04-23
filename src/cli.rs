use clap::{Command, Arg};

pub fn build_cli(name: &'static str, version: &'static str) -> Command<'static> {
    Command::new(name)
    .bin_name(name)
    .version(version)
    .author("Ferry Jérémie ferryjeremie@free.fr")
    .about("kill program")
    .arg_required_else_help(true)
    .arg(Arg::new("program").required(false).index(1))
}
