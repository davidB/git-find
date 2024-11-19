use std::path::Path;

use anyhow::{anyhow, Context};
use git2::{Remote, Repository, Status, StatusOptions};
use gtmpl::{self, Func, FuncError, Value};
use gtmpl_derive::Gtmpl;
use regex::Regex;
use slog::{info, trace, warn};
use std::collections::HashMap;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub struct Ctx {
    pub logger: slog::Logger,
}

#[derive(Clone, Gtmpl)]
pub struct GitRepo {
    path: Location,
    //repo: git2::Repository,
    remotes: Func,
    working_paths: Func,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct Location {
    full: String,
    file_name: String,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct RemoteData {
    name: String,
    url_full: String,
    url_host: String,
    url_path: String,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct WorkingPaths {
    untracked: Vec<String>,
    modified: Vec<String>,
    deleted: Vec<String>,
    added: Vec<String>,
    renamed: Vec<String>,
    conflicted: Vec<String>,
}

fn find_repo(args: &[Value]) -> Result<Repository, FuncError> {
    if let Value::Object(ref o) = &args[0] {
        let full = o
            .get("path")
            .and_then(|v| {
                if let Value::Object(ref o) = v {
                    o.get("full").map(|s| s.to_string())
                } else {
                    None
                }
            })
            .ok_or(anyhow!("path.full not empty"))?;
        let repo = Repository::open(Path::new(&full)).unwrap();
        Ok(repo)
    } else {
        Err(anyhow!("GitRepo required, got: {:?}", args).into())
    }
}

fn find_remotes(args: &[Value]) -> Result<Value, FuncError> {
    let repo = find_repo(args)?;
    let mut remotes = HashMap::new();
    repo.remotes()
        .unwrap()
        .iter()
        .filter_map(|x| x.and_then(|name| repo.find_remote(name).map(RemoteData::from).ok()))
        .for_each(|rd| {
            remotes.insert(rd.name.clone(), rd);
        });
    Ok(remotes.into())
}

fn find_working_paths(args: &[Value]) -> Result<Value, FuncError> {
    let repo = find_repo(args)?;
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    let statuses = repo
        .statuses(Some(&mut opts))
        .context("find status of working path")?;
    let mut untracked = vec![];
    let mut modified = vec![];
    let mut added = vec![];
    let mut deleted = vec![];
    let mut renamed = vec![];
    let mut conflicted = vec![];
    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            //eprintln!("path : {} {:?}", path, entry.status());
            let status = entry.status();
            if status.intersects(Status::INDEX_MODIFIED) || status.intersects(Status::WT_MODIFIED) {
                modified.push(path.to_owned());
            }
            if status.intersects(Status::INDEX_NEW) {
                added.push(path.to_owned())
            }
            if status.intersects(Status::WT_NEW) {
                untracked.push(path.to_owned())
            }
            if status.intersects(Status::INDEX_DELETED) || status.intersects(Status::WT_DELETED) {
                deleted.push(path.to_owned())
            }
            if status.intersects(Status::INDEX_RENAMED) || status.intersects(Status::WT_RENAMED) {
                renamed.push(path.to_owned())
            }
            if status.intersects(Status::CONFLICTED) {
                conflicted.push(path.to_owned())
            }
        }
    }
    Ok(WorkingPaths {
        untracked,
        modified,
        added,
        deleted,
        renamed,
        conflicted,
    }
    .into())
}
impl<'a> From<&'a Path> for GitRepo {
    //TODO manage result & error
    fn from(path: &Path) -> Self {
        GitRepo {
            path: Location {
                full: path.to_str().map(|x| x.to_owned()).unwrap(),
                file_name: path
                    .file_name()
                    .and_then(|x| x.to_str())
                    .map(|x| x.to_owned())
                    .unwrap(),
            },
            remotes: find_remotes,
            working_paths: find_working_paths,
        }
    }
}

impl<'b> From<Remote<'b>> for RemoteData {
    fn from(v: Remote) -> Self {
        // let host = url_parsed.host_str().unwrap_or("").to_owned();
        // let path = url_parsed.path().to_owned();
        let (host, path) = v
            .url()
            .map(|url| extract_host_and_path(url))
            .unwrap_or((None, None));
        RemoteData {
            name: v.name().unwrap_or("no_name").to_owned(),
            url_full: v.url().unwrap_or("").to_owned(),
            url_host: host.unwrap_or_else(|| "".to_owned()),
            url_path: path.unwrap_or_else(|| "".to_owned()),
        }
    }
}

/// url data extractor for git remote url (ssh, http, https)
fn extract_host_and_path(v: &str) -> (Option<String>, Option<String>) {
    let http_re = Regex::new(
        r"^https?://(?P<host>[[:alnum:]\._-]+)(:\d+)?/(?P<path>[[:alnum:]\._\-/]+).git$",
    )
    .unwrap();
    let ssh_re =
        Regex::new(r"^git@(?P<host>[[:alnum:]\._-]+):(?P<path>[[:alnum:]\._\-/]+).git$").unwrap();
    ssh_re
        .captures(v)
        .or_else(|| http_re.captures(v))
        .map(|caps| (Some(caps["host"].to_owned()), Some(caps["path"].to_owned())))
        .unwrap_or((None, None))
}

pub fn find_repos(ctx: &Ctx, root: &Path) -> Vec<GitRepo> {
    info!(ctx.logger, "find repositories"; "root" => &root.to_str());
    let mut found = vec![];
    let mut it = WalkDir::new(root).into_iter();
    loop {
        let entry = match it.next() {
            Option::None => break,
            Some(Err(err)) => {
                warn!(ctx.logger, "fail to access"; "err" => format!("{:?}", err));
                continue;
            }
            Some(Ok(entry)) => entry,
        };
        if is_hidden(&entry) {
            if entry.file_type().is_dir() {
                it.skip_current_dir();
            }
            continue;
        }
        if is_gitrepo(&entry) {
            found.push(GitRepo::from(entry.path()));
            it.skip_current_dir();
            continue;
        }
    }
    found
}

pub fn render(ctx: &Ctx, tmpl: &str, value: &GitRepo) -> String {
    trace!(ctx.logger, "render");
    //TODO remove the clone() and provide Value for &GitRepo
    gtmpl::template(tmpl, value.clone()).expect("template")
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_gitrepo(entry: &DirEntry) -> bool {
    entry.path().is_dir() && {
        let p = entry.path().join(".git").join("config");
        p.exists() && p.is_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn extract_host_and_path_on_ssh() {
        let v = "git@github.com:davidB/git-find.git";
        let (host, path) = extract_host_and_path(v);
        assert_that!(&host).is_equal_to(Some("github.com".to_owned()));
        assert_that!(&path).is_equal_to(Some("davidB/git-find".to_owned()));
    }

    #[test]
    fn extract_host_and_path_on_https() {
        let v = "https://github.com/davidB/git-find.git";
        let (host, path) = extract_host_and_path(v);
        assert_that!(&host).is_equal_to(Some("github.com".to_owned()));
        assert_that!(&path).is_equal_to(Some("davidB/git-find".to_owned()));
    }

    #[test]
    fn extract_host_and_path_on_http() {
        let v = "http://github.com/davidB/git-find.git";
        let (host, path) = extract_host_and_path(v);
        assert_that!(&host).is_equal_to(Some("github.com".to_owned()));
        assert_that!(&path).is_equal_to(Some("davidB/git-find".to_owned()));
    }
}
