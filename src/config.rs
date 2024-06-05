use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub timeout: u64,
}
