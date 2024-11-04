use std::path::PathBuf;

use clap::{arg, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    #[command(subcommand)]
    Repo(RepoCommands),
}

#[derive(Subcommand, Debug)]
pub enum RepoCommands {
    List,
    Clone,
    Pull,
}
