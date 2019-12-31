extern crate git2;
extern crate curl;
extern crate derive_more;

use std::path::PathBuf;
use std::io::Write;
use std::fs::File;

use self::derive_more::{Display, From};

use self::git2::Repository;
use self::git2::build::RepoBuilder;

use self::curl::easy::Easy;

fn config_path() -> PathBuf {
    let mut config_dir = dirs::config_dir().expect("cannot open config dir");
    config_dir.push("gitmoji-rust");

    config_dir
}

#[derive(Debug, From, Display)]
pub enum RetrievingError {
    Git(git2::Error),
    Curl(curl::Error),
    IO(std::io::Error),
}

pub struct Url {
    repo_url: String,
    json_url: String,
    branch:   String,
}

impl Url {
    pub fn github(repo: Option<&str>, branch: Option<&str>, path: Option<&str>) -> Url {
        let repo = match repo {
            Some(x) => x,
            None => "carloscuesta/gitmoji"
        };
        let branch = match branch {
            Some(x) => x,
            None => "master"
        };
        let path = match path {
            Some(x) => x,
            None => "src/data/gitmojis.json"
        };

        Url {
            repo_url: format!("https://github.com/{}.git", repo),
            json_url: format!("https://raw.githubusercontent.com/{}/{}/{}", repo, branch, path),
            branch:   branch.to_owned(),
        }
    }

    pub fn default_github() -> Url {
        Url::github(None, None, None)
    }
}

fn need_update(url: &Url) -> Result<bool, RetrievingError> {
    //path to the "gitmoji" git repo
    let mut repo_path = config_path();
    repo_path.push("gitmoji");
    let repo_path = repo_path;

    //if repo has to be cloned, we need to force the JSON download
    let mut force_json_dl = false;

    let repo = if repo_path.is_dir() {
        Repository::open_bare(&repo_path)
    } else {
        force_json_dl = true;
        RepoBuilder::new()
            .bare(true)
            .clone(&url.repo_url, &repo_path)
    }?;

    //OID of the local master branch
    let mut master = repo.find_reference(&format!("refs/heads/{}", url.branch))?;
    let local_oid = master.peel_to_commit()?.id();

    //fetch updates
    repo.find_remote("origin")?.fetch(&[&url.branch], None, None)?;

    //OID of the remote master branch
    let remote_oid = repo.find_reference(&format!("refs/remotes/origin/{}", url.branch))?
        .peel_to_commit()?
        .id();

    if local_oid == remote_oid {
        return Ok(force_json_dl);
    }

    //fast-forward the local branch to the remote
    master.set_target(remote_oid, "")?;

    Ok(true)
}

pub fn update(url: &Url) -> Result<(), RetrievingError> {
    if !need_update(url)? {
        return Ok(());
    }

    let mut json_path = config_path();
    json_path.push("gitmoji.json");
    let json_path = json_path;

    let mut file = File::create(&json_path)?;

    let mut curl = Easy::new();
    curl.url(&url.json_url)?;

    let mut transfer = curl.transfer();
    transfer.write_function(|data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;
    transfer.perform()?;

    Ok(())
}
