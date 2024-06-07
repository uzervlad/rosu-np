use serde_derive::Deserialize;

macro_rules! generate_structs {
  ($struct_name:ident, $partial_name:ident, $($name:ident: $type:ty [$token:literal],)+) => {
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

      pub fn get_keys() -> Vec<&'static str> {
        vec![$($token,)*]
      }
    }

    #[derive(Deserialize)]
    pub struct $partial_name {
      $(
        #[serde(rename = $token)]
        $name: Option<$type>,
      )*
    }
  };
}

generate_structs! {
  GameData, PartialGameData,

  artist: String ["artistRoman"],
  title: String ["titleRoman"],
  version: String ["diffName"],
  creator: String ["creator"],
  mods: String ["mods"],
  skin: String ["skin"],
  map_id: u32 ["mapid"],

  pp_98: f32 ["osu_98PP"],
  pp_99: f32 ["osu_99PP"],
  pp_ss: f32 ["osu_SSPP"],

  pp_mods_98: f32 ["osu_m98PP"],
  pp_mods_99: f32 ["osu_m99PP"],
  pp_mods_ss: f32 ["osu_mSSPP"],

  gamemode: String ["gameMode"],

  pp_mania: f32 ["mania_1_000_000PP"],
  pp_mods_mania: f32 ["mania_m1_000_000PP"],
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
    match (self.gamemode.as_str(), self.pp_mania == self.pp_mods_mania, self.pp_ss == self.pp_mods_ss) {
      ("OsuMania", true, _) => format!("PP (1m): {:.1}", self.pp_mania),
      ("OsuMania", false, _) => format!("PP (1m): {:.1} +{} {:.1}", self.pp_mania, self.get_mods(), self.pp_mods_mania),
      (_, _, true) => format!("PP (98/99/100): {:.1}/{:.1}/{:.1}", self.pp_98, self.pp_99, self.pp_ss),
      (_, _, false) => format!("PP (98/99/100): {:.1}/{:.1}/{:.1} +{} {:.1}/{:.1}/{:.1}", self.pp_98, self.pp_99, self.pp_ss, self.get_mods(), self.pp_mods_98, self.pp_mods_99, self.pp_mods_ss),
    }
  }

  pub fn get_skin(&self) -> String {
    self.skin.clone()
  }
}
