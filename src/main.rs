extern crate git_find;

use git_find::*;

fn main() {
    let root = ".";
    let tmpl = "{{.}}";
    find_repos(root)
        .iter()
        .map(|r| render(tmpl, r))
        .for_each(|s| println!("{}", s));
}
