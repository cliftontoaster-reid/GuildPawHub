use std::{path::PathBuf, str::FromStr};

use clap::{crate_authors, crate_version, Arg, Command};

#[tokio::main]
async fn main() {
  let comm = Command::new("guild_pawhub")
    .subcommands([Command::new("new").args([
      Arg::new("path").short('p').required(true),
      Arg::new("name").short('n').required(false),
    ])])
    .author(crate_authors!())
    .version(crate_version!())
    .get_matches();

  match comm.subcommand() {
    Some(("new", subcommand)) => {
      let path_str: String = subcommand.get_one::<&str>("path").unwrap().to_string();
      let path: PathBuf = PathBuf::from_str(&path_str).unwrap();
      let name = subcommand
        .get_one::<&str>("name")
        .unwrap_or(subcommand.get_one::<&str>("path").unwrap())
        .to_string();

      
    }
    _ => unimplemented!("Sowwy"),
  }
}
