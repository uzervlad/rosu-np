use std::{sync::Arc, time::Duration};

use futures_util::{SinkExt, StreamExt};
use tokio::{sync::Mutex, time::sleep};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::data::GameData;

const INIT_MESSAGE: &'static str = r#"[
  "artistRoman",
  "titleRoman",
  "diffName",
  "creator",
  "mapid",
  "skin"
]"#;

pub async fn sc_thread(game_data: Arc<Mutex<GameData>>) {
  let restart_timeout = Duration::from_secs(2);

  let url = "ws://localhost:20727/tokens";

  loop {
    let ws_stream = match connect_async(url).await {
      Ok((ws_stream, _)) => ws_stream,
      Err(e) => {
        println!("Unable to connect to StreamCompanion: {}", e);
        println!("Reconnecting...");
        sleep(restart_timeout).await;
        continue;
      }
    };

    let (mut ws_write, ws_read) = ws_stream.split();

    if let Err(e) = ws_write.send(Message::Text(INIT_MESSAGE.to_owned())).await {
      println!("Failed to initialize StreamCompanion tokens: {}", e);
      println!("Reconnecting...");
      sleep(restart_timeout).await;
      continue;
    }

    println!("Connected to StreamCompanion");

    let ws_game_data = game_data.clone();
    ws_read.for_each(|message| async {
      if let Ok(message) = message {
        if let Message::Text(data) = message {
          let new_data = serde_json::from_str::<GameData>(&data).unwrap();
          let mut game_data = ws_game_data.lock().await;
          game_data.update(new_data);
        }
      }
    }).await;

    println!("Disconnected from StreamCompanion");
    println!("Reconnecting...");
    sleep(restart_timeout).await;
  }
}