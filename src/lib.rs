#[macro_use]
pub extern crate slog;
extern crate gtmpl;
#[macro_use]
extern crate gtmpl_derive;
extern crate git2;
extern crate gtmpl_value;
extern crate regex;
extern crate walkdir;

#[cfg(test)]
#[macro_use]
extern crate spectral;

use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub struct Ctx {
    pub logger: slog::Logger,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct GitRepo {
    path: Location,
    remotes: HashMap<String, RemoteData>,
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

impl<'a> From<&'a Path> for GitRepo {
    //TODO manage result & error
    fn from(path: &Path) -> Self {
        let mut remotes = HashMap::new();
        let repo = git2::Repository::open(path).unwrap();
        repo.remotes()
            .unwrap()
            .iter()
            .filter_map(|x| {
                x.and_then(|name| {
                    repo.find_remote(name)
                        .map(|remote| RemoteData::from(remote))
                        .ok()
                })
            })
            //.collect::<Vec<_>>()
            //.into_iter()
            .for_each(|rd| {
                remotes.insert(rd.name.clone(), rd);
            });
        GitRepo {
            path: Location {
                full: path.to_str().map(|x| x.to_owned()).unwrap(),
                file_name: path.file_name()
                    .and_then(|x| x.to_str())
                    .map(|x| x.to_owned())
                    .unwrap(),
            },
            remotes,
        }
    }
}

impl<'b> From<git2::Remote<'b>> for RemoteData {
    fn from(v: git2::Remote) -> Self {
        // let host = url_parsed.host_str().unwrap_or("").to_owned();
        // let path = url_parsed.path().to_owned();
        let (host, path) = v.url()
            .map(|url| extract_host_and_path(url))
            .unwrap_or((None, None));
        RemoteData {
            name: v.name().unwrap_or("no_name").to_owned(),
            url_full: v.url().unwrap_or("").to_owned(),
            url_host: host.unwrap_or("".to_owned()),
            url_path: path.unwrap_or("".to_owned()),
        }
    }
}

/// url data extractor for git remote url (ssh, http, https)
fn extract_host_and_path(v: &str) -> (Option<String>, Option<String>) {
    let http_re = Regex::new(
        r"^https?://(?P<host>[[:alnum:]\._-]+)(:\d+)?/(?P<path>[[:alnum:]\._\-/]+).git$",
    ).unwrap();
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
            None => break,
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
    //TODO remove the clone() and provide Value for &GitRepo
    gtmpl::template(tmpl, value.clone()).expect("template")
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
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
