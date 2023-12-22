pub mod model;
use std::{path::Path, io::Write};

use model::{channel::ChannelConfig, internal_id::gen_id};
use serde::{Deserialize, Serialize};
use tokio::{
  fs,
  io::{AsyncReadExt, AsyncWriteExt},
};
use toml::{from_str, to_string_pretty};
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
}

impl Manifest {
  #[instrument]
  pub async fn load(base: &Path) -> Self {
    let path = base.join("manifest.toml");
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

    false
  }

  pub async fn save(&self, base: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Save the manifest
    fs::File::create(base.join("manifest.toml"))
      .await?
      .write_all(to_string_pretty(&self.manifest).unwrap().as_bytes())
      .await?;

    // Save the channels
    let mut root: Vec<ChannelConfig> = Vec::new();
    let _ = self.channels.iter().map(|c| match c {
      ConfigLocation::Root(r) => root.push(r.clone()),
      ConfigLocation::Directory((filename, data)) => {
        let mut f = std::fs::File::create(base.join(format!("channels.d/{}.toml", filename))).unwrap();
        f.write_all(to_string_pretty(&data).unwrap().as_bytes()).unwrap();
      }
    });

    fs::File::create(base.join("channels.toml"))
      .await?
      .write_all(to_string_pretty(&root).unwrap().as_bytes())
      .await?;

    Ok(())
  }
}

impl Default for Project {
  fn default() -> Self {
    use crate::model::channel::ChannelType::{Text, Voice};
    use crate::ConfigLocation::Root;

    Self {
      manifest: Manifest {
        name: "Fluffy boy".to_string(),
        image_conversion: true,
        animated_server_image: false,
      },
      channels: vec![
        Root(ChannelConfig {
          name: "fluffy-text-channel".to_string(),
          id: gen_id(),
          parent: None,
          channel_type: Text,
        }),
        Root(ChannelConfig {
          name: "Fluffy voice channel".to_string(),
          id: gen_id(),
          parent: None,
          channel_type: Voice,
        }),
      ],
    }
  }
}

#[derive(Debug)]
pub enum ConfigLocation<T> {
  Root(T),
  Directory((String, T)),
}
