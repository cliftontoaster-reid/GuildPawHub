pub mod cache;
pub mod model;
pub mod utils;
use std::path::Path;

use clap::crate_version;
use model::{category::CategoryConfig, channel::ChannelConfig, internal_id::gen_id};
use serde::{Deserialize, Serialize};
use serde_yaml::{from_str, to_string};
use tokio::{
  fs,
  io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::instrument;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
  /// The name of the server
  pub name: String,
  /// Should it upload an animated or static version of the image.
  ///
  /// Static can be from the following formats
  /// - `png`
  /// - `jpg`
  /// - `webp`
  ///
  /// Animated images must be `.gif` files.
  ///
  /// Files formats will be converted,
  /// images will be converted to `webp` or `gif` images upon upload, for efficiency.
  pub animated_server_image: bool,
  /// If the images should be converted to `webp` or `gif`.
  pub image_conversion: bool,
  pub version: String,
}

impl Manifest {
  #[instrument]
  pub async fn load(base: &Path) -> Self {
    let path = base.join("manifest.yaml");
    let mut str = String::new();
    fs::File::open(&path)
      .await
      .unwrap()
      .read_to_string(&mut str)
      .await
      .unwrap();

    from_str(&str).unwrap()
  }
}

#[derive(Debug)]
pub struct Project {
  pub manifest: Manifest,
  pub channels: Vec<ConfigLocation<ChannelConfig>>,
  pub categories: Vec<ConfigLocation<CategoryConfig>>,
}

impl Project {
  pub fn id_exists(&self, id: &str) -> bool {
    // Checking channels
    for n in &self.channels {
      match n {
        ConfigLocation::Root(c) => {
          if c.id == id {
            return true;
          }
        }
        ConfigLocation::Directory(c) => {
          if c.1.id == id {
            return true;
          }
        }
      }
    }
    // Checking categories
    for n in &self.categories {
      match n {
        ConfigLocation::Root(c) => {
          if c.id == id {
            return true;
          }
        }
        ConfigLocation::Directory(c) => {
          if c.1.id == id {
            return true;
          }
        }
      }
    }

    false
  }

  pub fn get_ids(&self) -> Vec<String> {
    let mut ids = Vec::new();
    #[cfg(debug_assertions)]
    println!("Data : {:#?}\n-----------", self);

    for n in &self.channels {
      match n {
        ConfigLocation::Root(c) => ids.push(c.id.clone()),
        ConfigLocation::Directory(c) => ids.push(c.1.id.clone()),
      }

      #[cfg(debug_assertions)]
      println!("ChannelConfig : {:?}\nIDs : {:?}\n-----------", n, ids);
    }
    // Checking categories
    for n in &self.categories {
      match n {
        ConfigLocation::Root(c) => ids.push(c.id.clone()),
        ConfigLocation::Directory(c) => ids.push(c.1.id.clone()),
      }

      #[cfg(debug_assertions)]
      println!("CategoryConfig : {:?}\nIDs : {:?}\n-----------", n, ids);
    }

    ids
  }

  pub async fn save(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Save the manifest
    fs::File::create(base.join("manifest.yaml"))
      .await?
      .write_all(to_string(&self.manifest).unwrap().as_bytes())
      .await?;

    ConfigLocation::<ChannelConfig>::save(&self.channels, base).await;
    ConfigLocation::<CategoryConfig>::save(&self.categories, base).await;

    Ok(())
  }

  pub async fn load(base: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    Ok(Self {
      manifest: Manifest::load(base).await,
      channels: ConfigLocation::<ChannelConfig>::load(base).await,
      categories: ConfigLocation::<CategoryConfig>::load(base).await,
    })
  }
}

impl Default for Project {
  fn default() -> Self {
    use crate::model::channel::ChannelType::{Text, Voice};
    use crate::ConfigLocation::Root;

    let top_id = gen_id();
    Self {
      manifest: Manifest {
        name: "Fluffy boy".to_string(),
        image_conversion: true,
        animated_server_image: false,
        version: crate_version!().to_string(),
      },
      channels: vec![
        Root(ChannelConfig {
          name: "fluffy-text-channel".to_string(),
          id: gen_id(),
          parent: top_id.clone(),
          channel_type: Text,
        }),
        Root(ChannelConfig {
          name: "Fluffy voice channel".to_string(),
          id: gen_id(),
          parent: top_id.clone(),
          channel_type: Voice,
        }),
      ],
      categories: vec![ConfigLocation::Root(CategoryConfig {
        name: "Genewal channels".to_string(),
        id: top_id.clone(),
      })],
    }
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ConfigLocation<T> {
  Root(T),
  Directory((String, T)),
}
