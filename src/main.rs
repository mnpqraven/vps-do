use actions::{remote_repo::clone_single, repo_list::repo_list};
use clap::Parser;
use utils::args::{CliArgs, Commands, RepoCommands};

mod actions;
mod utils;

fn main() {
    let cli = CliArgs::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Repo(RepoCommands::List)) => {
            let list = repo_list();
            dbg!(list);
        }
        Some(Commands::Repo(RepoCommands::Install)) => {
            // for now clone
            // TODO: parent fn handling deciding between clone or pull
            let list = repo_list();
            for slug in list {
                clone_single(&slug);
            }
            println!("cloning done");
        }
        None => {}
    }

    // Continued program logic goes here...
}
