extern crate git2;
extern crate dirs;

use git2::Repository;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    return config_dir;
}

fn gitmoji_path() -> PathBuf {
    let mut path = config_path();
    path.push("gitmoji");

    return path;
}

fn main() {
    let url = "https://github.com/carloscuesta/gitmoji/";

    let mut gitmoji_dir = gitmoji_path();

    let repo = match Repository::clone(url, gitmoji_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
