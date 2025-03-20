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
    let config_path = args.config.clone().unwrap_or("config.toml".to_string());

    let mut conf = match conf::load_config(config_path, args.verbose) {
        Some(c) => c,
        None => {
            let conf = cli::questions();
            println!("Config not found using defaults: {}", conf);
            conf
        }
    };
    let project_name = conf.project.name.lower().replace_whitespace('-');

    match &args.command {
        Commands::Create { project }=> {
            let a = if !project_name.is_empty() {
                project_name.as_str()
            } else {
                project.as_deref().unwrap_or("default")
            };
            cli::create(&args,&mut conf, a)
        }
        Commands::Print => cli::print(&args, &conf),
        Commands::Debug => cli::debug(&args, &conf),
        Commands::Remove { project } => {
            let project = project.as_deref().unwrap_or("default");
            cli::remove_project(&args, project)
        }
    }
}

