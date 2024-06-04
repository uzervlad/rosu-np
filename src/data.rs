use serde_derive::Deserialize;

#[derive(Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
  mods: Option<String>,
  artist_roman: Option<String>,
  title_roman: Option<String>,
  diff_name: Option<String>,
  creator: Option<String>,
  skin: Option<String>,
  mapid: Option<u32>,
}

impl GameData {
  pub fn update(&mut self, new_data: GameData) {
    if let Some(mods) = new_data.mods {
      self.mods = Some(mods);
    }
    if let Some(artist_roman) = new_data.artist_roman {
      self.artist_roman = Some(artist_roman);
    }
    if let Some(title_roman) = new_data.title_roman {
      self.title_roman = Some(title_roman);
    }
    if let Some(diff_name) = new_data.diff_name {
      self.diff_name = Some(diff_name);
    }
    if let Some(creator) = new_data.creator {
      self.creator = Some(creator);
    }
    if let Some(skin) = new_data.skin {
      self.skin = Some(skin);
    }
    if let Some(mapid) = new_data.mapid {
      self.mapid = Some(mapid);
    }
  }

  pub fn get_beatmap_string(&self) -> String {
    match (&self.artist_roman, &self.title_roman, &self.diff_name, &self.creator, self.mapid) {
      (Some(artist_roman), Some(title_roman), Some(diff_name), Some(creator), Some(mapid)) => 
        if mapid != 0 {
          format!("{} - {} [{}] by {} https://osu.ppy.sh/b/{}", artist_roman, title_roman, diff_name, creator, mapid)
        } else {
          format!("{} - {} [{}] by {}", artist_roman, title_roman, diff_name, creator)
        },
      _ => String::new(),
    }
  }

  pub fn get_skin(&self) -> String {
    match &self.skin {
      Some(skin) => skin.clone(),
      None => String::new(),
    }
  }
}
