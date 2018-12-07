extern crate dirs;
extern crate git2;

use git2::Repository;
//use git2::build::CheckoutBuilder;
use std::path::PathBuf;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    config_dir
}

fn gitmoji_path() -> PathBuf {
    let mut path = config_path();
    path.push("gitmoji");

    path
}

fn update_emojis(url: &str, repo_dir: &PathBuf) -> Result<(), git2::Error> {
    let repo = match Repository::open(&repo_dir) {
        Ok(r) => r,
        Err(_) => Repository::clone(url, &repo_dir)?
    };

    let mut origin = repo.find_remote("origin")?;
    origin.fetch(&["master"], None, None)?;
    /*
    origin.update_tips(None, true, git2::AutotagOption::Unspecified, None)?;
    let c = repo.find_branch("origin/master", git2::BranchType::Remote)?.get().peel_to_commit()?;

    repo.checkout_tree(c.as_object(), Some(CheckoutBuilder::new().force()))?;
    repo.set_head("FETCH_HEAD")?;
    */
    let b = repo.find_branch("origin/master", git2::BranchType::Remote)?.get();
    let c = repo.reference_to_annotated_commit(&b)?;
    repo.merge(&[&c], None, None)?;
    let s = git2::Signature::now("a", "a@a.com");
    repo.commit(Some("HEAD"), &s, &s, "merge", );
    repo.cleanup_state()?;

    Ok(())
}

fn main() {
    let url = "https://github.com/carloscuesta/gitmoji/";
    let repo_dir = gitmoji_path();

    update_emojis(url, &repo_dir).unwrap();
}
