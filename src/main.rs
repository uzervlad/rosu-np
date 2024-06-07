use std::{fs::File, io::BufReader, sync::Arc};

use config::Config;
use data::GameData;
use tokio::sync::Mutex;
use twitch_login::twitch_login;
use twitch_thread::twitch_thread;
use sc_thread::sc_thread;

#[cfg(not(debug_assertions))]
use updates::check_for_updates;

mod config;
mod data;
mod ratelimit;
mod sc_thread;
mod twitch_thread;
mod twitch_login;
mod updates;

#[tokio::main]
async fn main() {
  #[cfg(not(debug_assertions))]
  check_for_updates().await;
  
  let config = match File::open("config.json") {
    Ok(file) => {
      let reader = BufReader::new(file);
      serde_json::from_reader::<BufReader<File>, Config>(reader).expect("Failed to read config.json")
    }
    Err(_) => match twitch_login().await {
      Ok(config) => config,
      Err(e) => {
        println!("Something horrible happened: {}", e);
        return
      }
    },
  };

  let game_data = Arc::new(Mutex::new(GameData::default()));

  let chat_handle = {
    let chat_game_data = game_data.clone();
    tokio::spawn(async move {
      twitch_thread(&config, chat_game_data).await
    })
  };

  let sc_handle = {
    let sc_game_data = game_data.clone();
    tokio::spawn(async move {
      sc_thread(sc_game_data).await
    })
  };

  let _ = tokio::join!(chat_handle, sc_handle);
}
