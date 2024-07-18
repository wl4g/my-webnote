pub mod api;

use std::{ collections::HashMap, sync::OnceLock };
use core::panic;
use clap::{ ArgMatches, Command };

type SubcommandBuildFn = fn() -> Command;
type SubcommandHandleFn = fn(&ArgMatches) -> ();

static SUBCOMMAND_MAP: OnceLock<
  HashMap<&'static str, (SubcommandBuildFn, SubcommandHandleFn)>
> = OnceLock::new();

pub fn register_subcommand_handles() -> &'static HashMap<
  &'static str,
  (SubcommandBuildFn, SubcommandHandleFn)
> {
  SUBCOMMAND_MAP.get_or_init(|| {
    let mut map = HashMap::new();
    map.insert("start", (
      // Type inference error, forced conversion need.
      api::build_cli as SubcommandBuildFn,
      api::handle_cli as SubcommandHandleFn,
    ));
    map
  })
}

pub fn execute_commands_app() -> () {
  let mut app = Command::new("MyWebnote API Server")
    .version("1.0.0")
    .author("James Wong")
    .about("MyWebnote (Excalidraw) Rust API server")
    .arg_required_else_help(true); // When no args are provided, show help.

  let subcommand_map = register_subcommand_handles();
  // Add to all subcommands.
  for (name, (build_fn, _)) in subcommand_map.iter() {
    app = app.subcommand(build_fn().name(name));
  }

  let matches = app.get_matches();

  // Handling to actual subcommand.
  match matches.subcommand() {
    Some((name, sub_matches)) => {
      if let Some(&(_, handler)) = subcommand_map.get(name) {
        tracing::info!("Executing subcommand: {}", name);
        handler(sub_matches);
      } else {
        panic!("Unknown subcommand: {}. Use --help for a list of available commands.", name);
      }
    }
    None => {
      tracing::info!("No subcommand was used. Available commands are:");
      for name in subcommand_map.keys() {
        tracing::info!("  {}", name);
      }
      tracing::info!("Use <command> --help for more information about a specific command.");
    }
  }
}
