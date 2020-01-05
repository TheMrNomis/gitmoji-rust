use std::path::PathBuf;

pub struct Settings {
    pub config_path : PathBuf,
    pub repo_path : PathBuf,
    pub json_path : PathBuf,
}

impl Settings {
    pub fn new(config_dir: Option<PathBuf>) -> Settings {
        let config_dir = match config_dir {
            Some(x) => x,
            None => {
                let mut dir = dirs::config_dir().expect("cannot open config dir");
                dir.push("gitmoji-rust");
                dir
            }
        };

        let mut repo_dir = config_dir.clone();
        repo_dir.push("gitmoji");

        let mut json_file = config_dir.clone();
        json_file.push("gitmoji.json");

        Settings {
            config_path: config_dir,
            repo_path: repo_dir,
            json_path: json_file,
        }
    }
}
