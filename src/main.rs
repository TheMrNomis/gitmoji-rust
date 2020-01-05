mod settings;
mod gitmoji;

use settings::Settings;

fn main() {
    let settings = Settings::new(None);

    gitmoji::update(&gitmoji::Url::default_github(), &settings.repo_path, &settings.json_path).unwrap();
}
