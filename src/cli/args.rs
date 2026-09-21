use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oxid")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Oxid CLI Tools and Runtime")]
pub struct Cli {
    #[arg(short = 'l', long, global = true, value_name = "LOCALE")]
    pub lang: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New {
        project_name: String,

        #[arg(short, long, default_value = ".")]
        destination: PathBuf,
    },

    Run {
        path: Option<PathBuf>,
    },
}
