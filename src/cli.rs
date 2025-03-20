use clap::builder::Str;
use survex::{MetaData, SurvexOptions, SurvexError, TrimAndLower, ErrorKind};
use std::io::Write;
use std::path::PathBuf;

use crate::conf::{self, EqPoint};
#[cfg(feature = "mnemo")]
use crate::mnemo::{NemoReader, NemoReaderOptions};
use crate::survex::SurvexProject;
use crate::args::Args;
use crate::conf::Config;

fn init_dir(path: &PathBuf) {
    if !path.exists(){
        std::fs::create_dir_all(path).unwrap();
    } else {
        println!("Path already exists");
    }
}
pub fn questions() -> Config{
    let mut config = conf::Config::default();
    let mut pname = String::new();
    let mut author = String::new();
    print!("Enter project name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut pname).unwrap();
    print!("Enter author name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut author).unwrap();

    config.project.name = pname.trim().to_lowercase();
    config.project.author = author.trim().to_string();
    config
}
pub fn create(args: &Args, config: &mut Config, project: &str) {
    if args.verbose { 
        println!("Created a project {:?}\nConfig: {}\nProject: {}", args, config, project);
    }

    let mut output_dir = std::env::current_dir().unwrap();
    if let Some(path) = &args.output {
        output_dir.push(path);
    }
    output_dir.push(project.to_lowercase().replace(" ", "-"));

    init_dir(&output_dir);
    let filename = format!("{}.svx", project.to_lowercase().replace(" ", "-"));
    output_dir.push(filename);
    let mut path = output_dir;
    let project = match from_mnemo(args, config){
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    survex::write_project(&project, &path);
    println!("Created project!");
    if args.verbose {
        let mut temp = path.clone();
        temp.pop();
        println!("Project saved to: {:?}", temp);
        survex::print_project(&project);
    }
    // Writing the config
    match conf::write_config(config, &mut path) {
        Ok(_) => println!("Config saved to: {:?}", path),
        Err(e) => println!("Error saving config: {}", e),
    }
}
pub fn print(args: &Args, config: &Config) {
    if args.verbose { 
        println!("Created a project {:?}\nConfig: {}", args, config);
        dbg!(config);
    }
    let project = match from_mnemo(args, config) {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };
    survex::print_project(&project);
}
pub fn debug(args: &Args, config: &Config) {
    println!("Created a debug {:?}", args);
    conf::print_config(config);
    if args.input.is_some() { print(args, config);}

    let point = EqPoint { from: "A".to_string(), to: "B".to_string() }; 
    let vec = vec![point];
    dbg!(&vec);
    match serde_yaml::to_string(&vec){
        Ok(s) => println!("Points: {}", s),
        Err(e) => println!("Error: {}", e),
    }
    
}
pub fn remove_project(args: &Args, project: &str) {
    if args.verbose { println!("Removing project with args: {:?}\nProject: {}", args, project); }
    let path = std::env::current_dir().unwrap().join(project);
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
        println!("Removed project: {}", project);
    } else {
        println!("Project not found: {}", project);
    }
}
#[cfg(feature = "mnemo")]
fn init_nemo(args: &Args, config: &Config) -> Result<NemoReader, SurvexError> {
    let filter = args.filter.clone().unwrap_or("MNEMO".to_string());
    let input = match args.input.clone() {
        Some(v) => v,
        None => {
            return Err(SurvexError::new("No input file provided").kind(ErrorKind::Input));
        }
    };

    let options = NemoReaderOptions::new().filter(filter);
    let reader = match NemoReader::new().options(&options).load(input) {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    Ok(reader)
}
#[cfg(feature = "mnemo")]
fn from_mnemo(args: &Args, config: &Config) -> Result<SurvexProject, SurvexError> {
    let project_name = config.project.name.clone().lower().replace_whitespace('-');

    let reader = init_nemo(args, config)?;
    
    let project = SurvexProject::from(&reader);
    let options = parse_options(args, config);
    Ok(project.name(project_name).author(&config.project.author).options(options))
}
pub fn parse_options(args: &Args, config: &Config) -> SurvexOptions {
    let mut options = SurvexOptions::default();
    if let Some(o) = &config.options {
        if let Some(fix) = &o.fix {
            let mfix = MetaData::new("fix", Some(fix.to_string().as_str()));
            options.push(mfix);
        }
        if let Some(entrances) = &o.entrance {
            for entrance in entrances {
                let temp = MetaData::new("entrance", Some(entrance));
                options.push(temp);
            }
        }
        if let Some(equate) = &o.equate {
            for point in &equate.points {
                let point = MetaData::new("equate", Some(&point.to_string()));
                options.push(point);
            }
        }
    }
    options
}