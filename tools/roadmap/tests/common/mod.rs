#![allow(dead_code)]

use roadmap::derive;
use roadmap::diagnostic::Diagnostic;
use roadmap::repo::{LoadOptions, Repo};
use roadmap::util::copy_dir;
use roadmap::validate;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct TempRepo {
    pub path: PathBuf,
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

pub fn valid_src() -> PathBuf {
    fixtures().join("valid")
}

pub fn schema_src() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../schema/fields.json")
}

pub fn materialize_valid() -> TempRepo {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "roadmap-test-{}-{}-{nanos}",
        std::process::id(),
        TEMP_COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&path).expect("temp dir");
    copy_dir(&valid_src(), &path).expect("copy fixture");
    let schema_dir = path.join("tools/schema");
    fs::create_dir_all(&schema_dir).expect("schema dir");
    fs::copy(schema_src(), schema_dir.join("fields.json")).expect("copy schema");
    TempRepo { path }
}

pub fn overlay(root: &Path, files: &[(&str, &str)]) {
    for (relative, content) in files {
        let path = root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("overlay parent");
        }
        fs::write(path, content.as_bytes()).expect("write overlay");
    }
}

pub fn load(root: &Path, allow_drafts: bool, index: Option<PathBuf>) -> Repo {
    Repo::load(
        root.to_path_buf(),
        LoadOptions {
            allow_drafts,
            index_path: index,
        },
    )
    .expect("load repo")
}

pub fn check_entries(root: &Path, allow_drafts: bool, index: Option<PathBuf>) -> Vec<Diagnostic> {
    let repo = load(root, allow_drafts, index);
    let derived = derive::build(&repo);
    let mut diagnostics = validate::run(&repo, &derived);
    diagnostics.extend(roadmap::generate::stale_diagnostics(&repo, &derived));
    diagnostics.sorted(false)
}

pub fn codes(entries: &[Diagnostic]) -> Vec<String> {
    let mut values: Vec<String> = entries.iter().map(|entry| entry.code.to_string()).collect();
    values.sort();
    values.dedup();
    values
}

pub fn has_code(entries: &[Diagnostic], code: &str) -> bool {
    entries.iter().any(|entry| entry.code == code)
}

pub fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_roadmap"))
}

pub fn run(root: &Path, args: &[&str]) -> std::process::Output {
    binary()
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .expect("run roadmap")
}

pub fn read(root: &Path, relative: &str) -> String {
    fs::read_to_string(root.join(relative)).expect("read")
}

pub fn write(root: &Path, relative: &str, content: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent");
    }
    fs::write(path, content).expect("write");
}

pub fn real_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}
