use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "oxid")]
#[command(version = "0.1.0")]
#[command(about = "Oxid CLI Tools and Runtime")]
pub struct Cli {
    #[arg(short = 'l', long, global = true, value_name = "LOCALE")]
    pub lang: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New { project_name: String },
    Run,
}
