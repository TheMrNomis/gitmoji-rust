extern crate simsearch;

mod settings;
mod gitmoji;

use settings::Settings;
use gitmoji::Gitmojis;
use simsearch::SimSearch;

fn main() {
    let settings = Settings::new(None);

    gitmoji::update(&gitmoji::Url::default_github(), &settings.repo_path, &settings.json_path).unwrap();

    let db = Gitmojis::load(&settings.json_path).unwrap();
    let mut engine: SimSearch<usize> = SimSearch::new();

    for i in 0..db.gitmojis.len() {
        let gitmoji = &db.gitmojis[i];

        engine.insert(i, &format!("{}: {}", gitmoji.name, gitmoji.description));
    }

    let results: Vec<usize> = engine.search("lip");
    for i in results {
        let gitmoji = &db.gitmojis[i];
        println!("{} - {}", gitmoji.emoji, gitmoji.description);
    }
}
