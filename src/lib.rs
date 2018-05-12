#[macro_use]
pub extern crate slog;
extern crate walkdir;

use std::path::Path;
use walkdir::DirEntry;
use walkdir::WalkDir;

pub struct Ctx {
    pub logger: slog::Logger,
}

#[derive(Debug, Clone)]
pub struct GitRepo {
    dir: DirEntry,
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
            found.push(GitRepo { dir: entry.clone() });
            it.skip_current_dir();
            continue;
        }
        //println!("{}", &entry.path().display());
    }
    found
}

pub fn render(ctx: &Ctx, tmpl: &str, value: &GitRepo) -> String {
    format!("{:?}", value)
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
