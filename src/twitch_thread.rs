use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};
use twitch_irc::{login::StaticLoginCredentials, message::ServerMessage, ClientConfig, SecureTCPTransport, TwitchIRCClient};

use crate::{config::Config, data::GameData, ratelimit::Ratelimiter};

pub async fn twitch_thread(config: &Config, game_data: Arc<Mutex<GameData>>) {
  let restart_timeout = Duration::from_secs(2);

  loop {
    let client_config = ClientConfig::new_simple(StaticLoginCredentials::new(
      config.username.clone(),
      Some(config.token.clone()),
    ));
    
    let (mut incoming_messages, client) =
      TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

    match client.join(config.username.clone()) {
      Ok(_) => {
        println!("Joined channel!");
      }
      _ => {
        println!("Failed to join channel");
        println!("Reconnecting...");
        sleep(restart_timeout).await;
        continue;
      }
    }

    let mut ratelimiter = Ratelimiter::new(config.timeout);
    let chat_game_data = game_data.clone();

    while let Some(server_message) = incoming_messages.recv().await {
      if let ServerMessage::Privmsg(message) = server_message {
        match message.message_text.as_str() {
          "!np" => {
            if !ratelimiter.trigger("np".to_owned()) {
              continue;
            }
            let game_data = chat_game_data.lock().await;
            if let Err(e) = client.say_in_reply_to(&message, game_data.get_beatmap_string()).await {
              println!("Failed to reply: {}", e);
            }
          },
          "!pp" => {
            if !ratelimiter.trigger("pp".to_owned()) {
              continue;
            }
            let game_data = chat_game_data.lock().await;
            if let Err(e) = client.say_in_reply_to(&message, game_data.get_pp_string()).await {
              println!("Failed to reply: {}", e);
            }
          },
          "!skin" => {
            if !ratelimiter.trigger("skin".to_owned()) {
              continue;
            }
            let game_data = chat_game_data.lock().await;
            if let Err(e) = client.say_in_reply_to(&message, game_data.get_skin()).await {
              println!("Failed to reply: {}", e);
            }
          },
          _ => (),
        }
      }
    }

    println!("Disconnected from Twitch");
    println!("Reconnecting...");
    sleep(restart_timeout).await;
  }
}