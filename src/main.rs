extern crate git_find;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

use git_find::*;
use slog::Drain;

fn init_log() -> slog::Logger {
    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());
    info!(log, "start"; "version" => env!("CARGO_PKG_VERSION"));
    log
}

fn main() {
    let log = init_log();

    let root = ".";
    let tmpl = "{{.}}";
    let ctx = Ctx { logger: log };
    find_repos(&ctx, root)
        .iter()
        .map(|r| render(&ctx, tmpl, r))
        .for_each(|s| println!("{}", s));
}
