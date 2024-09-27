use serde_derive::{Deserialize, Serialize};

use crate::source::DataSource;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub channel: Option<String>,
  pub source: DataSource,
  pub timeout: u64,
}
