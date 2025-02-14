extern crate libmnemo as mnemo;
extern crate libsurvex as survex;

mod args;
mod cli;
mod conf;

use clap::Parser;
use args::{Args, Commands};
use survex::TrimAndLower;

fn main() {
    let args = Args::parse();
    let conf = conf::load_config(args.config.as_deref(), args.verbose);
    let project_name = conf.project.name.lower().replace_whitespace('-');

    match &args.command {
        Commands::Create { project }=> {
            let a = if !project_name.is_empty() {
                project_name.as_str()
            } else {
                project.as_deref().unwrap_or("default")
            };
            cli::create(&args,&conf, a)
        }
        Commands::Print => cli::print(&args, &conf),
        Commands::Debug => cli::debug(&args, conf),
        Commands::Remove { project } => {
            let project = project.as_deref().unwrap_or("default");
            cli::remove_project(&args, project)
        }
    }
}

