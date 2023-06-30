pub struct TrackConfig {
  volume: f64,
  sound: PathBuf,
}
pub struct BackgroundTrack {
  track: TrackConfig,
  percent_cleared_min: u128,
  percent_cleared_max: u128,
  probability: u8,
}
pub struct SoundFont {
  move_sound: HashSet<u128, TrackConfig>,
  win_sound: TrackConfig,
  lose_sound: TrackConfig,
  background_music: Vec<BackgroundTrack>,
  title_music: TrackConfig,
}
pub struct TileFont {
  character_map: HashSet<u128, char>,
  color_map: HashSet<u128, Color>,
}
pub struct Config {
  tile_font: TileFont,
  sound_font: SoundFont,
  animation_speed: f64,
  master_volume: f64,
  master_volume_mute: bool,
  background_music_volume: f64,
  background_music_volume_mute: bool,
  sound_effects_volume: f64,
  sound_effects_volume_mute: bool,
}
impl Default for Config {}

pub struct DataManager {
  config: Config,
  data_dirs: Box<&[PathBuf]>,
  persistent_data_dir: PathBuf,
}

impl DataManager {
  const DEFAULT_PERSISTENT_DATA_DIR: &'static str = "~/.config/greed_rs/";
  pub fn new() -> Self {
    todo!()
  }
  pub fn from_data_dirs(
    persistent_data_dir: &PathBuf,
    data_dir: &[PathBuf],
  ) -> Result<Self, Box<dyn Error>> {
    todo!()
  }
  pub fn save_config(&self) -> Result<(), Box<dyn Error>> {
    todo!()
  }
  pub fn get_config(&self) -> &Config {
    self.config
  }
  pub fn get_config_mut(&mut self) -> &mut Config {
    &mut self.config
  }
}
