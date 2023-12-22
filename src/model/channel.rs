use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChannelConfig {
  pub name: String,
  pub id: String,
  pub parent: Option<String>,
  #[serde(rename = "type")]
  pub channel_type: ChannelType,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ChannelType {
  Text,
  Voice,
}