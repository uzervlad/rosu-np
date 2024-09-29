use std::collections::HashMap;

use strfmt::{strfmt, FmtError};

#[derive(Debug, Default, Clone, Copy)]
pub enum GameMode {
  #[default]
  Osu,
  Taiko,
  Catch,
  Mania,
}

impl Into<GameMode> for u8 {
  fn into(self) -> GameMode {
    match self {
      0 => GameMode::Osu,
      1 => GameMode::Taiko,
      2 => GameMode::Catch,
      3 => GameMode::Mania,
      _ => unreachable!(),
    }
  }
}

impl Into<u8> for GameMode {
  fn into(self) -> u8 {
    match self {
      GameMode::Osu => 0,
      GameMode::Taiko => 1,
      GameMode::Catch => 2,
      GameMode::Mania => 3,
    }
  }
}

impl ToString for GameMode {
  fn to_string(&self) -> String {
    match self {
      GameMode::Osu => "osu",
      GameMode::Taiko => "taiko",
      GameMode::Catch => "catch",
      GameMode::Mania => "mania",
    }.to_owned()
  }
}

macro_rules! generate_structs {
  ($struct_name:ident, $partial_name:ident, $($name:ident: $type:ty,)+) => {
    #[derive(Debug, Default)]
    pub struct $struct_name {
      $($name: $type,)*
    }

    impl $struct_name {
      pub fn update(&mut self, new_data: $partial_name) {
        $(
          if let Some($name) = new_data.$name {
            self.$name = $name;
          }
        )*
      }
    }

    #[derive(Default)]
    pub struct $partial_name {
      $(
        pub $name: Option<$type>,
      )*
    }
  };
}

generate_structs! {
  GameData, PartialGameData,

  artist: String,
  artist_unicode: String,
  title: String,
  title_unicode: String,
  version: String,
  creator: String,

  mods: String,
  skin: String,
  map_id: u32,

  stars: f32,
  cs: f32,
  ar: f32,
  od: f32,
  hp: f32,

  pp_95: f32,
  pp_96: f32,
  pp_97: f32,
  pp_98: f32,
  pp_99: f32,
  pp_ss: f32,

  gamemode: GameMode,
}

impl GameData {
  fn get_mods(&self) -> String {
    if self.mods.len() == 0 {
      "".to_owned()
    } else {
      format!("+{}",self.mods.split(',').collect::<String>())
    }
  }

  fn get_beatmap_link(&self) -> String {
    if self.map_id == 0 {
      "".to_owned()
    } else {
      format!("https://osu.ppy.sh/b/{}", self.map_id)
    }
  }

  pub fn get_game_mode(&self) -> GameMode {
    self.gamemode
  }

  pub fn get_formatted_string(&self, string: &str) -> Result<String, FmtError> {
    let vars = HashMap::from([
      ("artist".to_owned(), self.artist.clone()),
      ("artist_unicode".to_owned(), self.artist_unicode.clone()),
      ("title".to_owned(), self.title.clone()),
      ("title_unicode".to_owned(), self.title_unicode.clone()),
      ("version".to_owned(), self.version.clone()),
      ("creator".to_owned(), self.creator.clone()),

      ("mods".to_owned(), self.get_mods()),

      ("skin".to_owned(), self.skin.clone()),

      ("map_id".to_owned(), self.map_id.to_string()),
      ("link".to_owned(), self.get_beatmap_link()),

      ("stars".to_owned(), self.stars.round().to_string()),
      ("cs".to_owned(), self.cs.round().to_string()),
      ("ar".to_owned(), self.ar.round().to_string()),
      ("od".to_owned(), self.od.round().to_string()),
      ("hp".to_owned(), self.hp.round().to_string()),

      ("pp_95".to_owned(), self.pp_95.round().to_string()),
      ("pp_96".to_owned(), self.pp_96.round().to_string()),
      ("pp_97".to_owned(), self.pp_97.round().to_string()),
      ("pp_98".to_owned(), self.pp_98.round().to_string()),
      ("pp_99".to_owned(), self.pp_99.round().to_string()),
      ("pp_ss".to_owned(), self.pp_ss.round().to_string()),

      ("gamemode".to_owned(), self.gamemode.to_string()),
    ]);

    strfmt(string, &vars)
  }
}
