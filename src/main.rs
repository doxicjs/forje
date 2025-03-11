use clap::command;

mod commands;
mod services;
mod utils;
use commands::{query_cmd, query_exec};
use include_dir::{Dir, include_dir};

static TEMPLATES_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/templates");

fn main() {
  let matches = command!()
    .arg_required_else_help(true)
    .about("Hammering code for a polished foundation.")
    .subcommand(query_cmd())
    .get_matches();

  query_exec(matches.subcommand_matches("query"));
}
