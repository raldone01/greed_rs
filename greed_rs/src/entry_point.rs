use clap::{ArgGroup, Parser};
use std::path::PathBuf;

/// Defaults are loaded from the config directory.
/// --config config-dir -- stores defaults for theme, anim-speed, sound, motd, colors
/// --seed feck#12x12 -- load game from seed
/// --load file.greed -- load game from save file
/// --immersive-menu -- replaces the main menu screen with a more immersive menu
/// --rpg -- launches the greed adventure game
/// --gsh -- launches the greedy shell
/// highscores -- subcommand displays the highscores and exits
/// --theme <directory/tar.gz or builtin> -- sets the keybinds, sound, colors, chars
/// install -- subcommand --theme <directory/tar.gz> -- copies the theme to the config dir
/// --sound <true/false>
/// --man -- prints a short description of the game
/// --colors
/// verify <.greed file>
/// strip <.greed file> -o <outfile>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(group(
            ArgGroup::new("game mode")
                .args(&["rpg", "immersive-menu", "ed"]),
        ))]
#[clap(group(
            ArgGroup::new("init source")
                .args(&["seed", "load"]),
        ))]
struct Args {
  // TODO: Windows? Macos? Webassembly? LINUX: "/.config/greed_rs"
  #[clap(short, long = "config-dir", value_parser, value_name = "DIR")]
  config_dir: Option<PathBuf>,
  #[clap(long, value_parser = seed_value_parser, value_name = "SEED")]
  seed: Option<greed::Seed>,
  #[clap(long, value_parser, value_name = ".GREED-FILE")]
  load: Option<PathBuf>,
  #[clap(long = "immersive-menu", value_parser, default_value_t = false)]
  immersive_menu: bool,
  #[clap(long, value_parser, default_value_t = false)]
  rpg: bool,
  #[clap(short, long = "theme", value_parser, value_name = "DIR")]
  theme_dir: Option<PathBuf>,
  #[clap(long, value_parser)]
  sound: Option<bool>,
  #[clap(long, value_parser, default_value_t = false)]
  man: bool,
  #[clap(long, value_parser, default_value_t = false)]
  ed: bool,
  #[clap(long, value_parser, default_value_t = true)]
  colors: bool,
}

fn seed_value_parser(s: &str) -> Result<greed::Seed, String> {
  Seed::try_from(s).map_err(|err| err.to_string())
}

/*let cli = Args::parse();

if !cli.ed {
  todo!();
}*/
