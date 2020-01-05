mod settings;
mod gitmoji;

use settings::Settings;
use gitmoji::Gitmojis;

fn main() {
    let settings = Settings::new(None);

    gitmoji::update(&gitmoji::Url::default_github(), &settings.repo_path, &settings.json_path).unwrap();

    let gitmojis = Gitmojis::load(&settings.json_path).unwrap();
    for gitmoji in gitmojis.gitmojis {
        println!("{} - {}", gitmoji.emoji, gitmoji.description);
    }
}
