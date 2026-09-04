mod common;

use roadmap::derive;
use roadmap::generate;
use roadmap::repo::{LoadOptions, Repo};
use roadmap::validate;

#[test]
fn real_repo_parses_without_panic() {
    let root = common::real_root();
    let repo = Repo::load(root, LoadOptions::default()).expect("load real repo");
    let derived = derive::build(&repo);
    let _diagnostics = validate::run(&repo, &derived);
    let rendered = generate::render(&repo, &derived).expect("render");
    assert!(rendered.contains_key("ROADMAP.md"));
    assert!(rendered.contains_key("STATUS.md"));
    assert!(rendered.contains_key("generated/index.json"));
}
