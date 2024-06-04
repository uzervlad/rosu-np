use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub timeout: u64,
}
