use argh::FromArgs;
use eyre::OptionExt;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(FromArgs)]
/// Manage arbitrary bookmarks
struct Args {
    #[argh(positional)]
    /// title of a bookmark
    title: Option<String>,

    #[argh(switch, description = "remove bookmark", short = 'r')]
    remove: bool,

    #[argh(option, description = "bookmark to set", short = 's')]
    set: Option<String>,

    #[argh(switch, description = "list bookmarks", short = 'l')]
    list: bool,

    #[argh(option, description = "append to bookmark before opening", short = 'a')]
    append: Option<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct Config {
    bookmarks: HashMap<String, String>,
}

fn main() -> eyre::Result<()> {
    let args: Args = argh::from_env();

    let Some(mut dir) = dirs::config_dir() else {
        println!("configuration directory was not found");
        return Ok(());
    };

    dir = dir.join("bo/config.toml");

    let mut conf: Config = if let Ok(s) = std::fs::read_to_string(&dir) {
        toml::from_str(&s)?
    } else {
        Config::default()
    };

    if args.list {
        println!("My Bookmarks:");
        for bookmark in &conf.bookmarks {
            println!("{} {}", bookmark.0, bookmark.1);
        }
    }

    if let Some(mut title) = args.title {
        if let Some(bookmark) = args.set {
            conf.bookmarks.insert(title, bookmark);
            return write(&dir, &conf);
        }

        if args.remove {
            conf.bookmarks.remove(&title);
            return write(&dir, &conf);
        }

        let bookmark = match conf.bookmarks.get_mut(&title) {
            Some(b) => b,
            None => &mut ("https://".to_owned() + &title),
        };

        if let Some(a) = args.append {
            bookmark.push_str(&a);
        }

        #[cfg(target_os = "linux")]
        std::process::Command::new("xdg-open")
            .arg(bookmark)
            .status()
            .expect("failed to open bookmark");

        #[cfg(target_os = "macos")]
        std::process::Command::new("open")
            .arg(bookmark)
            .status()
            .expect("failed to open bookmark");

        #[cfg(target_os = "windows")]
        std::process::Command::new("start")
            .arg(bookmark)
            .status()
            .expect("failed to open bookmark");
    } else if !args.list {
        println!(
            r#"
Required positional arguments not provided:
    title

Run bo --help for more information"#
        );
    }

    Ok(())
}

fn write(dir: &PathBuf, conf: &Config) -> eyre::Result<()> {
    if !dir.exists() {
        std::fs::create_dir(dir.parent().unwrap())?;
    }

    std::fs::write(dir, toml::to_string(&conf)?)?;
    Ok(())
}
