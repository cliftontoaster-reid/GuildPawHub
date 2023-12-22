use std::{
  env::current_dir,
  path::PathBuf,
  str::FromStr,
};

use clap::{crate_authors, crate_version, Arg, Command};
use guild_pawhub::{utils::find_duplicates, Project};
use tokio::fs::create_dir;

#[tokio::main]
async fn main() {
  let comm = Command::new("guild_pawhub")
    .subcommands([
      Command::new("new")
        .about("Creates a new guild project.")
        .args([
          Arg::new("path")
            .short('p')
            .required(true)
            .help("The path to the new project."),
          Arg::new("name")
            .short('n')
            .required(false)
            .help("The name to the new project."),
        ]),
      Command::new("check")
        .about("Checks the projects to detects mistakes.")
        .args([Arg::new("path")
          .short('p')
          .help("The path to the project.")
          .required(false)]),
    ])
    .author(crate_authors!())
    .version(crate_version!())
    .subcommand_required(true)
    .get_matches();

  match comm.subcommand() {
    Some(("new", subcommand)) => {
      let path_str: String = subcommand.get_one::<String>("path").unwrap().to_owned();
      let path: PathBuf = PathBuf::from_str(&path_str).unwrap();
      if path.exists() {
        panic!("Projects aweady exists!!!")
      }
      let name = subcommand
        .get_one::<String>("name")
        .unwrap_or(subcommand.get_one::<String>("path").unwrap())
        .to_owned();

      let mut project = Project::default();
      project.manifest.name = name.clone();

      #[cfg(debug_assertions)]
      println!("{:?}", &project);

      create_dir(&path).await.unwrap();
      project.save(&path).await.unwrap();

      println!("Project {} created succesfully inside of {:?}", name, path);
    }
    Some(("check", subcommand)) => {
      let path = match subcommand.get_one::<String>("path") {
        Some(s) => PathBuf::from_str(s).unwrap(),
        None => current_dir().unwrap(),
      };
      let mut owo = None;
      if path.join("manifest.yaml").exists() {
        owo = Some(path)
      } else {
        if path.is_absolute() {
          let mut uwu = path.ancestors();
          while let Some(j) = uwu.next() {
            if j.join("manifest.yaml").exists() {
              owo = Some(j.to_path_buf())
            }
          }
        }
      }

      if owo.is_none() {
        panic!("Cannot find manifest in directory or parent direcoties.");
      }

      let projektoj = Project::load(&owo.unwrap()).await.unwrap();
      let ids = projektoj.get_ids();
      let dupl = find_duplicates(ids.clone());

      if !dupl.is_empty() {
        eprintln!("Some IDs are causing issues, the following IDs have been given to multiple objects : {:?}", dupl);
        panic!("FATAL ERROR")
      }

      for i in projektoj.channels {
        match i {
          guild_pawhub::ConfigLocation::Root(s) => {
            if (s.parent != "") && (!ids.contains(&s.parent)) {
              #[cfg(debug_assertions)]
              println!("ChannelConfig : {:?}\nIDs : {:?}\n-----------", s, ids);
              eprintln!(
                "The parent ID \"{}\" for channel \"{}\" isn't assigned to anything.",
                s.parent, s.id
              );
              panic!("FATAL ERROR")
            }
          }
          guild_pawhub::ConfigLocation::Directory(s) => {
            if (s.1.parent != "") && (!ids.contains(&s.1.parent)) {
              #[cfg(debug_assertions)]
              println!("ChannelConfig : {:?}\nIDs : {:?}\n-----------", s, ids);
              eprintln!(
                "The parent ID \"{}\" for channel \"{}\" isn't assigned to anything.",
                s.1.parent, s.1.id
              );
              panic!("FATAL ERROR")
            }
          }
        }
      }
    }
    None => panic!("Meany, u need subcommand..."),
    _ => unimplemented!("Sowwy"),
  }
}
