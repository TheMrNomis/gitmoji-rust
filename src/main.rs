extern crate dirs;
extern crate git2;
extern crate curl;
extern crate derive_more;

use std::path::PathBuf;
use std::io::Write;
use std::fs::File;

use derive_more::{Display, From};

use git2::Repository;
use git2::build::RepoBuilder;

use curl::easy::Easy;

#[derive(Debug, From, Display)]
enum RetrievingError {
    Git(git2::Error),
    Curl(curl::Error),
    IO(std::io::Error),
}

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    config_dir
}

fn clone_or_open_bare(url: &str, path: &PathBuf) -> Result<Repository, git2::Error> {
    if path.is_dir() {
        Repository::open_bare(path)
    } else {
        RepoBuilder::new()
            .bare(true)
            .clone(url, path)
    }
}

fn emojis_need_update() -> Result<bool, RetrievingError> {
    let gitmoji_url = "https://github.com/carloscuesta/gitmoji/";

    //path to the "gitmoji" git repo
    let mut repo_path = config_path();
    repo_path.push("gitmoji");
    let repo_path = repo_path;

    let repo = clone_or_open_bare(gitmoji_url, &repo_path)?;

    //OID of the local master branch
    let mut master = repo.find_reference("refs/heads/master")?;
    let local_oid = master.peel_to_commit()?.id();

    //fetch updates
    repo.find_remote("origin")?.fetch(&["master"], None, None)?;

    //OID of the remote master branch
    let remote_oid = repo.find_reference("refs/remotes/origin/master")?
                       .peel_to_commit()?
                       .id();

   if local_oid == remote_oid {
        return Ok(false);
   }

   //fast-forward the local branch to the remote
   master.set_target(remote_oid, "")?;

   Ok(true)
}


fn update_emojis() -> Result<(), RetrievingError> {
    if !emojis_need_update()? {
        return Ok(());
    }

    let mut json_path = config_path();
    json_path.push("gitmoji.json");
    let json_path = json_path;

    let mut file = File::create(&json_path)?;

    let mut curl = Easy::new();
    curl.url("https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json")?;

    let mut transfer = curl.transfer();
    transfer.write_function(|data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;
    transfer.perform()?;

    Ok(())
}

fn main() {
    update_emojis().unwrap();
}
