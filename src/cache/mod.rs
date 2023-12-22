use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Lock {
  pub guild_id: u64,
  pub channel_ids: ChannelIdCache,
  pub category_ids: CategoryIdCache,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChannelIdCache {
  pub internal_id: String,
  pub id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryIdCache {
  pub internal_id: String,
  pub id: u64,
}