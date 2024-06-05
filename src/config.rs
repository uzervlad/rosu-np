use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub timeout: u64,
}

impl Config {
  pub fn example() -> Self {
    Self {
      username: "username".to_owned(),
      token: "qwertyasdfgh123456".to_owned(),
      timeout: 5,
    }
  }
}
