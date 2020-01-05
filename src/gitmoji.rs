extern crate git2;
extern crate curl;
extern crate derive_more;
extern crate serde;
extern crate serde_json;

use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Read, Write};

use self::derive_more::{Display, From};

use self::git2::Repository;
use self::git2::build::RepoBuilder;

use self::curl::easy::Easy;

use self::serde::{Serialize, Deserialize};

#[derive(Debug, From, Display)]
pub enum RetrievingError {
    Git(git2::Error),
    Curl(curl::Error),
    IO(std::io::Error),
}

#[derive(Debug, From, Display)]
pub enum LoadError {
    IO(std::io::Error),
    JSON(self::serde_json::Error),
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

fn need_update(url: &Url, repo_path: &Path) -> Result<bool, RetrievingError> {
    //if repo has to be cloned, we need to force the JSON download
    let mut force_json_dl = false;

    let repo = if repo_path.is_dir() {
        Repository::open_bare(repo_path)
    } else {
        force_json_dl = true;
        RepoBuilder::new()
            .bare(true)
            .clone(&url.repo_url, repo_path)
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

pub fn update(url: &Url, repo_path: &Path, json_path: &Path) -> Result<(), RetrievingError> {
    if !need_update(url, repo_path)? {
        return Ok(());
    }

    let mut json_tmp_path = PathBuf::new();
    json_tmp_path.push(json_path);
    json_tmp_path.set_extension("json.new");
    let json_tmp_path = json_tmp_path;

    let mut file = File::create(&json_tmp_path)?;

    let mut curl = Easy::new();
    curl.url(&url.json_url)?;

    let mut transfer = curl.transfer();
    transfer.write_function(|data| {
        file.write_all(data).unwrap();
        Ok(data.len())
    })?;
    transfer.perform()?;

    std::fs::rename(json_tmp_path, json_path)?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gitmoji {
    pub emoji:  String,
    pub entity: Option<String>,
    pub code:   String,
    pub description: String,
    pub name:   String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gitmojis {
    pub gitmojis: Vec<Gitmoji>,
}

impl Gitmojis {
    pub fn load(file: &Path) -> Result<Gitmojis, LoadError> {
        let mut json = String::new();
        File::open(file)?.read_to_string(&mut json)?;
        let ret: Gitmojis = serde_json::from_str(&json)?;

        Ok(ret)
    }
}
