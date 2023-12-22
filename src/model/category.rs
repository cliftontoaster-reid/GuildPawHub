use std::{io::Write, path::Path};

use serde::{Deserialize, Serialize};
use serde_yaml::{to_string, from_str};
use tokio::{fs::{create_dir, self, read_dir, File}, io::{AsyncWriteExt, AsyncReadExt}};

use crate::ConfigLocation;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryConfig {
  pub name: String,
  pub id: String,
}

impl ConfigLocation<CategoryConfig> {
    pub async fn save(data: &Vec<ConfigLocation<CategoryConfig>>, base: &Path) {
      // Save the channels
      let mut root: Vec<CategoryConfig> = Vec::new();
      create_dir(base.join(format!("category.d"))).await.unwrap();
      for c in data {
        match c {
          ConfigLocation::Root(r) => {
            #[cfg(debug_assertions)]
            println!("{:?}", &r);
  
            root.push(r.clone())
          }
          ConfigLocation::Directory((filename, data)) => {
            let mut f =
              std::fs::File::create(base.join(format!("category.d/{}.yaml", filename))).unwrap();
            f.write_all(to_string(&data).unwrap().as_bytes()).unwrap();
          }
        }
      }
  
      if !root.is_empty() {
        fs::File::create(base.join("categories.yaml"))
          .await
          .unwrap()
          .write_all(to_string(&root).unwrap().as_bytes())
          .await
          .unwrap();
      }
    }

    pub async fn load(base: &Path) -> Vec<ConfigLocation<CategoryConfig>> {
      let mut owo = Vec::new();
      // Load base.
      let mut base_r = String::new();
      File::open(base.join("categories.yaml")).await.unwrap().read_to_string(&mut base_r).await.unwrap();
      for i in from_str::<Vec<CategoryConfig>>(&base_r).unwrap() {
        owo.push(Self::Root(i));
      }
  
      let mut uwu = read_dir(base.join("category.d")).await.unwrap();
      while let Some(i) = uwu.next_entry().await.unwrap() {
        if i.file_name().to_str().unwrap().ends_with(".yaml") {
          let mut dir_r = String::new();
          File::open(i.path()).await.unwrap().read_to_string(&mut dir_r).await.unwrap();
  
          owo.push(Self::Directory(from_str(&dir_r).unwrap()))
        }
      } 
  
      owo
    }
  }