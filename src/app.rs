use anyhow::Result;

use crate::cli::{Cli, Commands, NewCommands};

pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { path } => crate::scaffold::init_project(&path),
        Commands::New { command } => match command {
            NewCommands::Change { name, path } => crate::scaffold::new_change(&path, &name),
        },
        Commands::Render { target, path } => crate::render::render_target(&path, target),
        Commands::Validate { path } => crate::validate::validate_project(&path),
    }
}
