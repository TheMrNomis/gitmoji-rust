mod gitmoji;

use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    config_dir
}

fn main() {
    //path to the "gitmoji" git repo
    let mut repo_path = config_path();
    repo_path.push("gitmoji");
    let repo_path = repo_path;

    //json file path
    let mut json_path = config_path();
    json_path.push("gitmoji.json");
    let json_path = json_path;

    gitmoji::update(&gitmoji::Url::default_github(), &repo_path, &json_path).unwrap();
}
