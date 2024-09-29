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

    let channel = match config.channel.as_ref() {
      Some(channel) => channel.clone(),
      None => config.username.clone(),
    };

    match client.join(channel) {
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
            let template = config.get_template("np").unwrap();
            match game_data.get_formatted_string(&template) {
              Ok(reply) => {
                if let Err(e) = client.say_in_reply_to(&message, reply).await {
                  println!("Failed to reply: {}", e);
                }
              },
              Err(e) => {
                println!("Failed to get formatted string: {}", e);
              }
            }
          },
          "!pp" => {
            if !ratelimiter.trigger("pp".to_owned()) {
              continue;
            }
            let game_data = chat_game_data.lock().await;
            let template = config.get_template("pp").unwrap();
            match game_data.get_formatted_string(&template) {
              Ok(reply) => {
                if let Err(e) = client.say_in_reply_to(&message, reply).await {
                  println!("Failed to reply: {}", e);
                }
              },
              Err(e) => {
                println!("Failed to get formatted string: {}", e);
              }
            }
          },
          "!skin" => {
            if !ratelimiter.trigger("skin".to_owned()) {
              continue;
            }
            let game_data = chat_game_data.lock().await;
            let template = config.get_template("skin").unwrap();
            match game_data.get_formatted_string(&template) {
              Ok(reply) => {
                if let Err(e) = client.say_in_reply_to(&message, reply).await {
                  println!("Failed to reply: {}", e);
                }
              },
              Err(e) => {
                println!("Failed to get formatted string: {}", e);
              }
            }
          },
          // TODO: Custom commands
          _ => (),
        }
      }
    }

    println!("Disconnected from Twitch");
    println!("Reconnecting...");
    sleep(restart_timeout).await;
  }
}