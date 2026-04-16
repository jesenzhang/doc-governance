use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "doc-governance")]
#[command(version)]
#[command(about = "AI project documentation governance and OpenSpec-aware artifact generator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize governance and OpenSpec structure in a project
    Init {
        #[arg(long, default_value = ".")]
        path: String,
    },
    /// Create a new change workspace
    New {
        #[command(subcommand)]
        command: NewCommands,
    },
    /// Render deployable artifacts for a target coding agent
    Render {
        #[arg(long)]
        target: TargetAgent,
        #[arg(long, default_value = ".")]
        path: String,
    },
    /// Validate docs/spec/governance consistency
    Validate {
        #[arg(long, default_value = ".")]
        path: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum NewCommands {
    /// Create a new OpenSpec change folder
    Change {
        name: String,
        #[arg(long, default_value = ".")]
        path: String,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum TargetAgent {
    Codex,
    Claude,
    Opencode,
    Kilo,
}
