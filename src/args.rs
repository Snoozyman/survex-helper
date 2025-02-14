use clap::{Parser, Subcommand};

type OptString = Option<String>;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Survex helper program to transform Mnemo files to Survex format
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    /// Optional, output filename, defaults to out.svx
    #[arg(short, long)]
    pub output: Option<String>,

    /// Path to input files
    #[arg(short, long)]
    pub input: Option<String>,

    /// Path to config file
    #[arg(short, long)]
    pub config: Option<String>,

    /// Filter for mnemo files
    pub filter: Option<String>,

    /// Verbose mode
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,
}
pub enum InputType {
    Mnemo,
}
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a Survex project
    Create {
        /// Name of the project
        project: Option<String>,

    },
    Print,
    Debug,
    Remove {
        /// Name of the project
        project: Option<String>,
    },
}