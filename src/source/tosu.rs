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
#[serde(rename_all = "camelCase")]
struct TosuDataMenuBeatmapMetadata {
  artist: String,
  artist_original: String,
  title: String,
  title_original: String,
  mapper: String,
  difficulty: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct TosuDataMenuBeatmapStats {
  sr: f32,
  cs: f32,
  ar: f32,
  od: f32,
  hp: f32,
}

#[derive(Deserialize)]
struct TosuDataMenuBeatmap {
  id: u32,
  metadata: TosuDataMenuBeatmapMetadata,
  stats: TosuDataMenuBeatmapStats,
}

#[derive(Deserialize)]
struct TosuDataMenuMods {
  #[allow(dead_code)]
  num: u64,
  str: String,
}

#[derive(Deserialize)]
struct TosuDataMenuPp {
  #[serde(rename = "95")]
  _95: f32,
  #[serde(rename = "96")]
  _96: f32,
  #[serde(rename = "97")]
  _97: f32,
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
      artist_unicode: Some(self.menu.bm.metadata.artist_original),
      title: Some(self.menu.bm.metadata.title),
      title_unicode: Some(self.menu.bm.metadata.title_original),
      version: Some(self.menu.bm.metadata.difficulty),
      creator: Some(self.menu.bm.metadata.mapper),
      mods: Some(self.menu.mods.str),
      skin: Some(self.settings.folders.skin),
      map_id: Some(self.menu.bm.id),
      stars: Some(self.menu.bm.stats.sr),
      cs: Some(self.menu.bm.stats.cs),
      ar: Some(self.menu.bm.stats.ar),
      od: Some(self.menu.bm.stats.od),
      hp: Some(self.menu.bm.stats.hp),
      pp_95: Some(self.menu.pp._95),
      pp_96: Some(self.menu.pp._96),
      pp_97: Some(self.menu.pp._97),
      pp_98: Some(self.menu.pp._98),
      pp_99: Some(self.menu.pp._99),
      pp_ss: Some(self.menu.pp._100),
      gamemode: Some(self.menu.game_mode.into()),
      ..Default::default()
    }
  }
}

#[allow(unused_assignments)]
pub async fn thread(game_data: Arc<Mutex<GameData>>) {
  let restart_timeout = Duration::from_secs(2);

  let url = "ws://localhost:24050/ws";

  let mut reconnecting = false;

  loop {
    let ws_stream = match connect_async(url).await {
      Ok((ws_stream, _)) => ws_stream,
      Err(e) => {
        if !reconnecting {
          println!("Unable to connect to tosu: {}", e);
          println!("Reconnecting...");
          reconnecting = true;
        }
        tokio::time::sleep(restart_timeout).await;
        continue;
      }
    };

    let (_, ws_read) = ws_stream.split();

    reconnecting = false;
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
    reconnecting = true;
    tokio::time::sleep(restart_timeout).await;
  }
}