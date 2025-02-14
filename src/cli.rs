use std::path::PathBuf;

use survex::TrimAndLower;

use crate::conf;
use crate::mnemo::{NemoReader, NemoReaderOptions};
use crate::survex::SurvexProject;
use std::env::current_dir;
use crate::args::Args;
use crate::conf::Config;

pub fn create(args: &Args, config: &Config, project: &str) {
    if args.verbose { println!("Created a project {:?}\nConfig: {}\nProject: {}", args, config, project); }

    let mut output_dir = current_dir().unwrap();
    if let Some(path) = &args.output {
        output_dir.push(path);
    }
    output_dir.push(project.to_lowercase().replace(" ", "-"));

    survex::init_dir(&output_dir);
    let filename = format!("{}.svx", project.to_lowercase().replace(" ", "-"));
    output_dir.push(filename);
    let mut path = output_dir;
    let project = from_mnemo(args, config);
    dbg!(&project.name);
    survex::write_project(&project, &path);
    println!("Created project!");
    if args.verbose {
        let mut temp = path.clone();
        temp.pop();
        println!("Project saved to: {:?}", temp);
        survex::print_project(&project);
    }

    conf::write_config(config, &mut path);
}
pub fn print(args: &Args, config: &Config) {
    if args.verbose { println!("Created a project {:?}\nConfig: {}", args, config); }

    let project = from_mnemo(args, config);
    survex::print_project(&project);
}
pub fn debug(args: &Args, config: Config) {
    println!("Created a debug {:?}", args);
}
pub fn remove_project(args: &Args, project: &str) {
    if args.verbose { println!("Removed a project {:?}\nProject: {}", args, project); }
    let path = current_dir().unwrap().join(project);
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
        println!("Removed project: {}", project);
    } else {
        println!("Project not found: {}", project);
    }
}
fn init(args: &Args, config: &Config) -> (NemoReader, NemoReaderOptions) {
    // let path_buf = PathBuf::from(path);
    let filter = args.filter.clone().unwrap_or("MNEMO".to_string());
    let input = args.input.as_ref().unwrap();

    let options = NemoReaderOptions::new().filter(filter);
    let reader = match NemoReader::new().options(&options).load(input) {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let p = SurvexProject::from(&reader);

    (reader, options).to_owned()
}
fn from_mnemo(args: &Args, config: &Config) -> SurvexProject {
    let project_name = config.project.name.clone().lower().replace_whitespace('-');

    let (reader, options) = init(&args, &config);
    let p = SurvexProject::from(&reader);
    p.name(project_name).author(&config.project.author)
}