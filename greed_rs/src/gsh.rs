// Greedy shell
use clap::Parser;

/// --ed mode
/// Subcommands:
///   mv <wedsaq>/<left up right down>/<iolkju>/<lurd> -- case insensitive
///   cat -p --possible -- prints the field
///   rm <n> -- undos n moves
///   stat -- prints the current stats
///   uname -- prints version & about
///   history <n> -- prints the last n moves
///   cd <DIR> --load <FLAG> -- changes the config directory optionally does not load the config
///   write --file <opt FILE> --settings -- Saves the game and settings.
///   help -- prints the available commands
///   man -- prints the manual
///   set --sound off ... -- allows settings to be changed
///   fsck <FILE> -- validate a save file
///   strip <FILE> -o <OUTFILE> -- Removes unnecessary information from a save file.
///   ls -- lists all save files and shows the current config directory -- -a shows additional information -- --themes to show all themes -- --sounds to show all soundfonts
///   modprobe <seed>/random/<file>/<save_file_name> --name <NAME> -- starts/loads a new game
///   exit -f -w --force --write <opt file> -- closes the program one flag is required
#[derive(Parser, Debug)]
struct EdArgs {
  // TODO: only commands here
}