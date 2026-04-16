mod app;
mod cli;
mod render;
mod scaffold;
mod validate;

use anyhow::Result;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    app::run(cli)
}
