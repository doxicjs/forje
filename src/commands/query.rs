use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn cmd() -> clap::Command {
  return Command::new("query")
    .about("Forges Queries using Tanstack Query\nForges use{Name}.tsx file that exports the query")
    .arg(
      Arg::new("query-name")
        .short('n')
        .long("name")
        .required(true)
        .help("Hammers the name of the query"),
    )
    .arg(
      Arg::new("route-name")
        .short('r')
        .long("route")
        .help("Hammers the query with Route param"),
    )
    .arg(
      Arg::new("enabled-flag")
        .short('e')
        .long("enabled")
        .action(ArgAction::SetTrue)
        .requires("route-name")
        .help("Hammers the query to only run when Route is provided"),
    )
    .arg(
      Arg::new("params-flag")
        .short('p')
        .long("params")
        .action(ArgAction::SetTrue)
        .help("Hammers the query with Params Zod schema"),
    );
}

pub fn exec(matches: Option<&ArgMatches>) {
  let query_name = matches
    .unwrap()
    .try_get_one::<String>("query-name")
    .unwrap();

  if query_name.is_some() {
    println!("{}", query_name.unwrap());
  }

  let route_name = matches
    .unwrap()
    .try_get_one::<String>("route-name")
    .unwrap();

  if route_name.is_some() {
    println!("{}", route_name.unwrap());
  }

  let enabled_flag = matches.unwrap().get_one::<bool>("enabled-flag").unwrap();

  println!("{}", enabled_flag.to_string());

  let params_flag = matches.unwrap().get_one::<bool>("enabled-flag").unwrap();

  println!("{}", params_flag.to_string());
}
