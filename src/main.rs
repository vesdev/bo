use std::collections::HashMap;

use argh::FromArgs;
use eyre::OptionExt;
use serde::{Deserialize, Serialize};

#[derive(FromArgs)]
/// Simple input capture
struct Args {
    /// title of a bookmark
    #[argh(positional)]
    title: String,

    /// url of a bookmark to add
    #[argh(positional)]
    bookmark: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Config {
    bookmarks: HashMap<String, String>,
}

fn main() -> eyre::Result<()> {
    let args: Args = argh::from_env();

    let dir = dirs::config_dir()
        .ok_or_eyre("fonfiguration directory was not found")?
        .join("bo/config.toml");

    let mut conf: Config = if let Ok(s) = std::fs::read_to_string(&dir) {
        toml::from_str(&s)?
    } else {
        Config::default()
    };

    if let Some(bookmark) = args.bookmark {
        conf.bookmarks.insert(args.title, bookmark);

        if !dir.exists() {
            std::fs::create_dir(dir.parent().unwrap())?;
        }

        std::fs::write(dir, toml::to_string(&conf)?)?;
        return Ok(());
    }

    let bookmark = conf
        .bookmarks
        .get(&args.title)
        .ok_or_eyre(format!("bookmark '{}' does not exist", &args.title))?;

    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(bookmark)
        .output()
        .expect("failed to open bookmark");

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(bookmark)
        .output()
        .expect("failed to open bookmark");

    #[cfg(target_os = "windows")]
    std::process::Command::new("start")
        .arg(bookmark)
        .output()
        .expect("failed to open bookmark");

    Ok(())
}
