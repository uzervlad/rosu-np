use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::data::GameData;

pub mod tosu;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DataSource {
  Tosu,
  Rosu,
}

impl Default for DataSource {
  fn default() -> Self {
    Self::Tosu
  }
}

impl DataSource {
  pub fn create(&self, game_data: Arc<Mutex<GameData>>) -> JoinHandle<()> {
    let this = self.clone();

    tokio::spawn(async move {
      match this {
        Self::Tosu => tosu::thread(game_data).await,
        _ => unimplemented!("DataSource not implemented: {:?}", this),
      }
    })
  }
}