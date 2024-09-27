use std::{sync::Arc, time::Duration};

use futures_util::StreamExt;
use serde_derive::Deserialize;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::data::{GameData, PartialGameData};

#[derive(Deserialize)]
struct TosuDataSettingsFolders {
  skin: String,
}

#[derive(Deserialize)]
struct TosuDataSettings {
  folders: TosuDataSettingsFolders,
}

#[derive(Deserialize)]
struct TosuDataMenuBeatmapMetadata {
  artist: String,
  title: String,
  mapper: String,
  difficulty: String,
}

#[derive(Deserialize)]
struct TosuDataMenuBeatmap {
  id: u32,
  metadata: TosuDataMenuBeatmapMetadata,
}

#[derive(Deserialize)]
struct TosuDataMenuMods {
  #[allow(dead_code)]
  num: u64,
  str: String,
}

#[derive(Deserialize)]
struct TosuDataMenuPp {
  #[serde(rename = "98")]
  _98: f32,
  #[serde(rename = "99")]
  _99: f32,
  #[serde(rename = "100")]
  _100: f32,
}

#[derive(Deserialize)]
struct TosuDataMenu {
  #[serde(rename = "gameMode")]
  game_mode: u8,
  bm: TosuDataMenuBeatmap,
  mods: TosuDataMenuMods,
  pp: TosuDataMenuPp,
}

#[derive(Deserialize)]
struct TosuData {
  settings: TosuDataSettings,
  menu: TosuDataMenu,
}


impl Into<PartialGameData> for TosuData {
  fn into(self) -> PartialGameData {
    PartialGameData {
      artist: Some(self.menu.bm.metadata.artist),
      title: Some(self.menu.bm.metadata.title),
      version: Some(self.menu.bm.metadata.difficulty),
      creator: Some(self.menu.bm.metadata.mapper),
      mods: Some(self.menu.mods.str),
      skin: Some(self.settings.folders.skin),
      map_id: Some(self.menu.bm.id),
      pp_mods_98: Some(self.menu.pp._98),
      pp_mods_99: Some(self.menu.pp._99),
      pp_mods_ss: Some(self.menu.pp._100),
      gamemode: Some(self.menu.game_mode.into()),
      ..Default::default()
    }
  }
}

#[derive(Deserialize)]
struct TosuPPResponse {
  pp: f32,
}

async fn nomod_pp_thread(game_data: Arc<Mutex<GameData>>) {
  loop {
    let gamemode = game_data.lock().await.get_game_mode();
    let (_98, _99, _100) = tokio::join!(
      reqwest::get(format!("http://localhost:24050/api/calculate/pp?mode={}&acc=98", gamemode as u8)),
      reqwest::get(format!("http://localhost:24050/api/calculate/pp?mode={}&acc=99", gamemode as u8)),
      reqwest::get(format!("http://localhost:24050/api/calculate/pp?mode={}&acc=100", gamemode as u8))
    );

    match (_98, _99, _100) {
      (Ok(r98), Ok(r99), Ok(r100)) => {
        let (pp98, pp99, pp100) = tokio::join!(
          r98.json::<TosuPPResponse>(),
          r99.json::<TosuPPResponse>(),
          r100.json::<TosuPPResponse>(),
        );

        if let (Ok(r98), Ok(r99), Ok(r100)) = (pp98, pp99, pp100) {
          let data = PartialGameData {
            pp_98: Some(r98.pp),
            pp_99: Some(r99.pp),
            pp_ss: Some(r100.pp),
            ..Default::default()
          };

          let mut game_data = game_data.lock().await;
          game_data.update(data);
        }
      }
      _ => {}
    }

    tokio::time::sleep(Duration::from_secs(2)).await;
  }
}

pub async fn thread(game_data: Arc<Mutex<GameData>>) {
  let restart_timeout = Duration::from_secs(2);

  let url = "ws://localhost:24050/ws";

  let nm_game_data = game_data.clone();
  tokio::spawn(async move {
    nomod_pp_thread(nm_game_data).await
  });

  loop {
    let ws_stream = match connect_async(url).await {
      Ok((ws_stream, _)) => ws_stream,
      Err(e) => {
        println!("Unable to connect to tosu: {}", e);
        println!("Reconnecting...");
        tokio::time::sleep(restart_timeout).await;
        continue;
      }
    };

    let (_, ws_read) = ws_stream.split();

    println!("Connected to tosu");

    let ws_game_data = game_data.clone();
    ws_read.for_each(|message| async {
      if let Ok(Message::Text(data)) = message {
        let new_data = serde_json::from_str::<TosuData>(&data).unwrap();
        let mut game_data = ws_game_data.lock().await;
        game_data.update(new_data.into());
      }
    }).await;

    println!("Disconnected from tosu");
    println!("Reconnecting...");
    tokio::time::sleep(restart_timeout).await;
  }
}