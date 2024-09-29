use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::source::DataSource;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub username: String,
  pub token: String,
  pub channel: Option<String>,
  pub source: DataSource,
  pub timeout: u64,
  #[serde(default)]
  pub templates: HashMap<String, String>,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      username: String::new(),
      token: String::new(),
      channel: None,
      source: DataSource::default(),
      timeout: 5,
      templates: HashMap::default(),
    }
  }
}

impl Config {
  pub fn get_template(&self, name: &str) -> Option<String> {
    self.templates.get(name).map(|s| s.clone()).or_else(|| {
      match name {
        "np" => Some("{artist} - {title} [{version}] by {creator} {link}".to_string()),
        "pp" => Some("PP {mods} (98/99/100): {pp_98}/{pp_99}/{pp_ss}".to_string()),
        "skin" => Some("Skin: {skin}".to_string()),
        _ => None
      }
    })
  }
}
