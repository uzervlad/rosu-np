use std::{fs::File, io::BufReader, sync::Arc};

use rosu_np::config::Config;
use rosu_np::data::GameData;
use tokio::sync::Mutex;
use rosu_np::twitch_login::twitch_login;
use rosu_np::twitch_thread::twitch_thread;

#[cfg(not(debug_assertions))]
use rosu_np::updates::check_for_updates;

#[tokio::main]
async fn main() {
  #[cfg(not(debug_assertions))]
  check_for_updates().await;
  
  let config = match File::open("config.ron") {
    Ok(file) => {
      let reader = BufReader::new(file);
      ron::de::from_reader::<BufReader<File>, Config>(reader).expect("Failed to read config.json")
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

  let update_handle = {
    let update_game_data = game_data.clone();
    config.source.create(update_game_data)
  };

  let chat_handle = {
    let chat_game_data = game_data.clone();
    tokio::spawn(async move {
      twitch_thread(&config, chat_game_data).await
    })
  };

  let _ = tokio::join!(chat_handle, update_handle);
}
