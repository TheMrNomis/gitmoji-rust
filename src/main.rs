extern crate git2;
extern crate dirs;

use git2::Repository;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    return config_dir;
}

fn main() {
    let url = "https://github.com/carloscuesta/gitmoji/";

    let mut gitmoji_dir = config_path();
    gitmoji_dir.push("gitmoji");

    let repo = match Repository::clone(url, gitmoji_dir.to_str().expect("error")) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
