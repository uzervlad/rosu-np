use std::{fs::File, io::{BufReader, Write}, sync::Arc};

use config::Config;
use data::GameData;
use tokio::sync::Mutex;
use twitch_thread::twitch_thread;
use sc_thread::sc_thread;

#[cfg(not(debug_assertions))]
use updates::check_for_updates;

mod config;
mod data;
mod ratelimit;
mod sc_thread;
mod twitch_thread;
mod updates;

#[tokio::main]
async fn main() {
  let config = {
    let file = match File::open("config.json") {
      Ok(file) => file,
      Err(_) => {
        let mut file = File::create("config.json").expect("Unable to create an example config");

        let example_config = Config::example();
        file.write_all(serde_json::to_vec_pretty(&example_config).unwrap().as_slice()).expect("Unable to write example config");

        println!("An example config has been created!");

        return;
      },
    };

    let reader = BufReader::new(file);
    serde_json::from_reader::<BufReader<File>, Config>(reader).expect("Failed to read config.json")
  };

  #[cfg(not(debug_assertions))]
  check_for_updates().await;

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
