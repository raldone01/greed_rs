pub struct SoundFont {}
pub struct TileFont {}
pub struct TileColors {}

pub struct Theme {
  motd: Vec<String>,
  tile_font: TileFont,
  tile_colors: TileColors,
  sound_font: SoundFont,
  anim_speed: u8,
  volume: u8,
}

impl Default for Theme {}

pub struct Config {
  config_dir: std::path::PathBuf,
  base_theme: Theme,
}

pub struct SmartConfig {
  
}
