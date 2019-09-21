extern crate clap;
extern crate omnidux_core;

use clap::{App, Arg, SubCommand};
use omnidux_core::{config, generator, debug};

fn main() {
  let matches = App::new("omnidux")
    .version("0.1.0")
    .about("Does great things!")
    .author("Rene Eichhorn")
    .subcommand(
      SubCommand::with_name("generate")
        .arg(Arg::with_name("platform")
          .short("p")
          .long("platform")
          .help("Sets the target platform")
          .takes_value(true)
          .default_value("ios")
          .possible_values(&["ios"]))

        .arg(Arg::with_name("source")
          .short("s")
          .long("source")
          .help("Sets the source folder")
          .required(true)
          .takes_value(true))
    )
    .subcommand(
      SubCommand::with_name("debug")
        .arg(Arg::with_name("platform")
          .short("p")
          .long("platform")
          .help("Sets the target platform")
          .takes_value(true)
          .default_value("ios")
          .possible_values(&["ios"]))

        .arg(Arg::with_name("source")
          .short("s")
          .long("source")
          .help("Sets the source folder")
          .required(true)
          .takes_value(true))
    )
    .get_matches(); 

  if let Some(matches) = matches.subcommand_matches("generate") {
    // Parse configuration of project.
    let platform = matches.value_of("platform").unwrap();
    let source = matches.value_of("source").unwrap();
    let config_path = format!("{s}/omnidux.{p}.yaml", s = source, p = platform);
    let config = config::build_config_from_file(config_path).unwrap();

    // Generate main library.
    generator::rust::RustProjectBuilder::new(config.name.clone())
      .set_destination(format!("{d}/.omnidux", d = source))
      .build()
      .unwrap();

    print!("{:#?}", config);
  } else if let Some(matches) = matches.subcommand_matches("debug") {
    // Parse configuration of project.
    let platform = matches.value_of("platform").unwrap();
    let source = matches.value_of("source").unwrap();
    let config_path = format!("{s}/omnidux.{p}.yaml", s = source, p = platform);
    let config = config::build_config_from_file(config_path).unwrap();

    // Run debug executor
    debug::run_debugger(&config);
  }
}
