mod gitmoji;

fn main() {
    gitmoji::update(&gitmoji::Url::default_github()).unwrap();
}
