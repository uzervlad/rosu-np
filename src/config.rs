use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub channel: Option<String>,
  pub timeout: u64,
}
