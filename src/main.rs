use clap::command;

mod commands;
use commands::{query_cmd, query_exec};

fn main() {
  let matches = command!()
    .about("Hammering code for a polished foundation.")
    .subcommand(query_cmd())
    .get_matches();

  query_exec(matches.subcommand_matches("query"));
}
