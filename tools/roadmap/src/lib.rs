pub mod assign;
pub mod commands;
pub mod config;
pub mod coverage;
pub mod derive;
pub mod diagnostic;
pub mod fmt;
pub mod generate;
pub mod graph;
pub mod model;
pub mod parser;
pub mod repo;
pub mod schema;
pub mod util;
pub mod validate;

use crate::derive::Derived;
use crate::diagnostic::Diagnostics;
use crate::repo::{LoadOptions, Repo};
use anyhow::Result;
use std::path::PathBuf;

pub fn analyze(root: PathBuf, options: LoadOptions) -> Result<(Repo, Derived, Diagnostics)> {
    let repo = Repo::load(root, options)?;
    let derived = derive::build(&repo);
    let diagnostics = validate::run(&repo, &derived);
    Ok((repo, derived, diagnostics))
}
