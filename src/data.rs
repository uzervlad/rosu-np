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
  title: String,
  version: String,
  creator: String,
  mods: String,
  skin: String,
  map_id: u32,

  pp_98: f32,
  pp_99: f32,
  pp_ss: f32,

  pp_mods_98: f32,
  pp_mods_99: f32,
  pp_mods_ss: f32,

  gamemode: GameMode,
}

impl GameData {
  fn get_mods(&self) -> String {
    self.mods.split(',').collect::<String>()
  }

  pub fn get_beatmap_string(&self) -> String {
    if self.map_id != 0 {
      format!("{} - {} [{}] by {} https://osu.ppy.sh/b/{}", self.artist, self.title, self.version, self.creator, self.map_id)
    } else {
      format!("{} - {} [{}] by {}", self.artist, self.title, self.version, self.creator)
    }
  }

  pub fn get_pp_string(&self) -> String {
    if self.pp_ss == self.pp_mods_ss {
      format!("PP (98/99/100): {:.1}/{:.1}/{:.1}", self.pp_98, self.pp_99, self.pp_ss)
    } else {
      format!("PP (98/99/100): {:.1}/{:.1}/{:.1} +{} {:.1}/{:.1}/{:.1}", self.pp_98, self.pp_99, self.pp_ss, self.get_mods(), self.pp_mods_98, self.pp_mods_99, self.pp_mods_ss)
    }
  }

  pub fn get_game_mode(&self) -> GameMode {
    self.gamemode
  }

  pub fn get_skin(&self) -> String {
    self.skin.clone()
  }
}
