use std::{fs::File, io::BufReader, sync::Arc};

use config::Config;
use data::GameData;
use futures_util::{SinkExt, StreamExt};
use ratelimit::Ratelimiter;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use twitch_irc::{login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport, TwitchIRCClient};

mod config;
mod data;
mod ratelimit;

#[tokio::main]
async fn main() {
  let config = {
    let file = File::open("config.json").expect("config.json doesn't exist");
    let reader = BufReader::new(file);
    serde_json::from_reader::<BufReader<File>, Config>(reader).unwrap()
  };

  let client_config = ClientConfig::new_simple(
    StaticLoginCredentials::new(
      config.username.clone(),
      Some(config.token.clone())
    )
  );

  let game_data = Arc::new(Mutex::new(GameData::default()));

  let (mut incoming_messages, twitch_client) = 
    TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

  twitch_client.join(config.username).unwrap();

  let mut ratelimiter = Ratelimiter::new(config.timeout);
  let chat_game_data = game_data.clone();
  let chat_handle = tokio::spawn(async move {
    while let Some(server_message) = incoming_messages.recv().await {
      match server_message {
        ServerMessage::Privmsg(message) => {
          match message.message_text.as_str() {
            "!np" => {
              if !ratelimiter.trigger("np".to_owned()) {
                continue;
              }
              let game_data = chat_game_data.lock().await;
              twitch_client.say_in_reply_to(&message, game_data.get_beatmap_string()).await.unwrap();
            },
            "!skin" => {
              if !ratelimiter.trigger("skin".to_owned()) {
                continue;
              }
              let game_data = chat_game_data.lock().await;
              twitch_client.say_in_reply_to(&message, game_data.get_skin()).await.unwrap();
            },
            _ => (),
          }
        },
        _ => (),
      }
    }
  });

  let url = "ws://localhost:20727/tokens";

  let (ws_stream, _) = connect_async(url).await.unwrap();

  let (mut ws_write, ws_read) = ws_stream.split();

  ws_write.send(Message::Text(r#"["artistRoman", "titleRoman", "diffName", "creator", "mapid", "skin"]"#.to_owned())).await.unwrap();

  let ws_game_data = game_data.clone();
  let ws_handle = ws_read.for_each(|message| async {
    if let Ok(message) = message {
      if let Message::Text(data) = message {
        let new_data = serde_json::from_str::<data::GameData>(&data).unwrap();
        let mut game_data = ws_game_data.lock().await;
        game_data.update(new_data);
      }
    }
  });

  println!("Initialized?");

  ws_handle.await;
  chat_handle.await.unwrap();
}
