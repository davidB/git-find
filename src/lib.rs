#[macro_use]
pub extern crate slog;
#[macro_use]
extern crate gtmpl;
#[macro_use]
extern crate gtmpl_derive;
extern crate gtmpl_value;
extern crate walkdir;

use gtmpl::Value;
use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub struct Ctx {
    pub logger: slog::Logger,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct GitRepo {
    path: Location,
}

#[derive(Debug, Clone, Gtmpl)]
pub struct Location {
    full: String,
    file_name: String,
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
            found.push(GitRepo {
                path: Location {
                    full: entry.path().to_str().map(|x| x.to_owned()).unwrap(),
                    file_name: entry.file_name().to_str().map(|x| x.to_owned()).unwrap(),
                },
            });
            it.skip_current_dir();
            continue;
        }
        //println!("{}", &entry.path().display());
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
        let p = entry.path().join(".git");
        p.exists() && p.is_dir()
    }
}
