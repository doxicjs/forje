use crate::services::template_engine::render_template;
use clap::{Arg, ArgAction, ArgMatches, Command};
use tera::Context;

pub fn cmd() -> clap::Command {
  return Command::new("query")
    .about("Forges Queries using Tanstack Query\nForges use{Name}.tsx file that exports the query")
    .arg(
      Arg::new("query_name")
        .short('n')
        .long("name")
        .required(true)
        .help("Hammers the name of the query"),
    )
    .arg(
      Arg::new("route_name")
        .short('r')
        .long("route")
        .help("Hammers the query with Route param"),
    )
    .arg(
      Arg::new("params_name")
        .short('p')
        .long("params")
        .help("Hammers the query with Params Zod schema"),
    )
    .arg(
      Arg::new("is_enabled")
        .short('e')
        .long("enabled")
        .action(ArgAction::SetTrue)
        .requires("route_name")
        .help("Hammers the query to only run when Route is provided"),
    );
}

pub fn exec(matches: Option<&ArgMatches>) {
  let mut context = Context::new();

  if matches.is_none() {
    return;
  }

  let query_name = matches
    .unwrap()
    .try_get_one::<String>("query_name")
    .unwrap()
    .unwrap();

  let route_name = matches
    .unwrap()
    .try_get_one::<String>("route_name")
    .unwrap();

  if route_name.is_some() {
    context.insert("route_name", &route_name.unwrap().to_string());
  }

  let params_name = matches
    .unwrap()
    .try_get_one::<String>("params_name")
    .unwrap();

  if params_name.is_some() {
    context.insert("params_name", &params_name.unwrap().to_string());
  }

  let is_enabled = matches.unwrap().get_one::<bool>("is_enabled").unwrap();

  context.insert("is_enabled", &is_enabled);

  context.insert("query_name", &query_name.to_string());

  render_template(&"query/base.tera", &context)
}
