#[macro_use]
pub extern crate slog;

pub struct Ctx {
    pub logger: slog::Logger,
}

pub struct GitRepo {}

pub fn find_repos(ctx: &Ctx, root: &str) -> Vec<GitRepo> {
    unimplemented!()
}

pub fn render(ctx: &Ctx, tmpl: &str, value: &GitRepo) -> String {
    unimplemented!()
}
