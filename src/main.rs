use std::fs;
use std::path::PathBuf;

use clap::Parser;
use slog::{debug, info, o, trace, Drain};

use git_find::*;

#[derive(Parser, Debug)]
#[clap(version, about, author = "davidB")]
struct Cmd {
    // The number of occurences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    /// print on stderr
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    /// format of the output
    /// print on stdout
    #[arg(
        short = 't',
        long = "tmpl",
        default_value = "{{with .working_paths}}{{if .conflicted}}C{{else}} {{end}}{{if .modified}}M{{else}}{{if .added}}M{{else}}{{if .deleted}}M{{else}}{{if .renamed}}M{{else}} {{end}}{{end}}{{end}}{{end}}{{if .untracked}}U{{else}} {{end}}{{end}}\t{{ .path.file_name }}\t{{ .path.full }}\t{{with .remotes.origin}} {{ .name }} {{.url_full}} {{end}}"
    )]
    format: String,

    /// root directory of the search
    #[arg(value_name = "DIR", default_value = ".")]
    dir: PathBuf,
}

fn init_log(level_min: slog::Level) -> slog::Logger {
    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain)
        .build()
        .filter_level(level_min)
        .fuse();
    let log = slog::Logger::root(drain, o!());
    info!(log, "start"; "version" => env!("CARGO_PKG_VERSION"));
    debug!(log, "debug enabled");
    trace!(log, "trace enabled");
    log
}

fn main() {
    let cmd = Cmd::parse();
    let log_level =
        slog::Level::from_usize(3 + cmd.verbose as usize).unwrap_or(slog::Level::Warning);
    let log = init_log(log_level);

    let root = fs::canonicalize(&cmd.dir).unwrap();
    let tmpl = &cmd.format;
    let ctx = Ctx { logger: log };
    find_repos(&ctx, &root)
        .iter()
        .map(|r| render(&ctx, tmpl, r))
        .for_each(|s| println!("{}", s));
}
