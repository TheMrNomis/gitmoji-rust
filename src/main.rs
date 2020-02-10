extern crate simsearch;
extern crate termion;

mod settings;
mod gitmoji;

use settings::Settings;
use gitmoji::Gitmojis;
use simsearch::SimSearch;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use std::io::{Read, Write};

fn main() {
    let settings = Settings::new(None);

    gitmoji::update(&gitmoji::Url::default_github(), &settings.repo_path, &settings.json_path).unwrap();

    let db = Gitmojis::load(&settings.json_path).unwrap();
    let mut engine: SimSearch<usize> = SimSearch::new();

    for i in 0..db.gitmojis.len() {
        let gitmoji = &db.gitmojis[i];

        engine.insert(i, &format!("{}: {}", gitmoji.name, gitmoji.description));
    }

    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();

    let mut search = String::new();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => break,//TODO
            Key::Ctrl('c') => break, //TODO
            Key::Char('\n') => break,
            Key::Char(c) => search.push(c),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    let results: Vec<usize> = engine.search(search.as_str());
    for i in results {
        let gitmoji = &db.gitmojis[i];
        println!("{} - {}", gitmoji.emoji, gitmoji.description);
    }
}
