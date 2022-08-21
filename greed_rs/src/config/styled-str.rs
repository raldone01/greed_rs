/// # itfs - Insane terminal formatting standart
/// ▼
/// TODO: Use ascii unit seperator instead of §?
/// 
/// * `§` is the escape char
/// * `§§` is just the `§` char
/// * `§s` pushes the current style onto the style stack (SCOPED LOCAL STR ONLY)
/// * `§r` pops the last style from the style stack and overwrites the current style (SCOPED LOCAL STR ONLY)
/// * `§<fg>:<bg>:<style>[:{<fmt>}<resource_id>]§`
/// 
/// Any color or style changes are applied additively.
/// 
/// * `§{<fmt>}<resource_id>§` inserts an application specific resource into the string.
/// Formattable with the default rust formatting options.
/// If the resource implements a custom formatter the `<fmt>` string is passed directly to it.
/// If you are only inserting a resource the colons at the start are optional.
/// If a format AND either `<fg>` or `<bg>` or `<style>` is provided the additional formatting is local for the resource 
/// and does not modify the stream style.
/// TODO: ALLOW STYLING STRIP!
/// 
/// `<resource_id>` represent an external data type provided by the application.
/// resource_id format:
/// * Resource ids must not contain a `.` and must be `a-zA-Z0-9_`
/// * Builtin have no prefix (`b.` can be used)
/// * The ids of the current theme are prefixed with `t.`
/// * Other themes are prefixed by their name `themes.other_theme.some_resource`
/// 
/// Refer to https://docs.rs/tui/0.6.0/tui/style/enum.Color.html for more information.
/// `<fg>` and `<bg>` are color values.
/// The following color values are supported:
/// * `rst` | `reset` - resets to the terminals default color
/// * `blk` | `black`
/// * `red`
/// * `grn` | `green`
/// * `yel` | `yellow`
/// * `blu | `blue`
/// * `mag` | `magenta`
/// * `cyn` | `cyan`
/// * `gry` | `gray`
/// * `lred` | `light_red`
/// * `lgrn` | `light_green`
/// * `lyel` | `light_yellow`
/// * `lblu` | `light_blue`
/// * `lmag` | `light_magenta`
/// * `lcyn` | `light_cyan`
/// * `wht` | `white`
/// * `(<HEX>,<HEX>,<HEX>)` - HEX = 0..=256
/// * `<HEX>` - Ansi color value as HEX = 0..=256
/// 
/// Refer to https://docs.rs/tui/0.6.0/tui/style/struct.Modifier.html for more information.
/// `<style>` is a list of styling flags seperated by `;`.
/// The following flags are supported:
/// `bld` | `bold`
/// `dim` | `dimmed`
/// `ita` | `italic`
/// `uln` | `underline`
/// `sblk` | `slow_blink`
/// `rblk` | `rapid_blink`
/// `rev` | `reversed` | `inverted` | `inv`
/// `hdn` | `hidden`
/// `x` | `crossed out`
/// 
/// Examples:
/// `§:::§` is a noop
/// `§red§` sets the foreground color to red.
/// `§:red§` sets the background color to red.
/// `§::ita;x§` sets the style to italic and crossed out.
/// 
/// `§§` prints just §
/// 
/// `§red§str1§s§blu§blu§rstr2` yields `<red>str1</red><blue>blu</blue><red>str2</red>`
/// 
/// 
/// TODO: Generic over LocalResourceProvider?
struct ItfsFormatter {
  
}

impl ItfsFormatter {
    fn fmt(fmt: &str) -> TermionString {
        todo!()
    }
}

/// TODO REPLACE
struct TermionString {

}

trait GlobalResourceProvider {
    fn lookup_resource(id: &ResourceID) -> Resource;
}

struct LocalResourceProvider {
    
}

impl LocalResourceProvider {
    fn new(namespace: &str) -> Self  { todo!() }
  fn lookup_resource(id: &ResourceID) -> Resource { todo!() }
}

struct ResourceID {

}

trait Resource {
 /// Returns the global resource id
 fn id() -> ResourceID;
}

trait ResourceFmt {
  fn fmt(fmt: &str) -> TermionString;
}

// NOTE: Use tui as color backend and crossterm as terminal impl.

// global resource enum
// let img: Image = engine.look_unwrap<Resource::IMAGE>("com.app.res.image");
