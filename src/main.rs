use crate::{
    deps::check_dependencies,
    download::download,
    prompts::{Config, run_dialog},
};
use std::process::exit;

mod cli;
mod deps;
mod download;
mod prompts;

fn main() {
    if let Err(e) = check_dependencies() {
        eprintln!(
            "To install: {}",
            e.dependency.install_instructions.for_current_os()
        );
        exit(1);
    }

    let config: Config = run_dialog();

    download(config);
}
