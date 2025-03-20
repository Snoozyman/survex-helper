use serde::{Deserialize, Serialize};
use survex::SurvexOptions;
use std::{fmt::{Display, Formatter}, path::PathBuf};
use survex::SurvexError;

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Config {
    pub project: Project,
    pub options: Option<Options>,
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub author: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
    pub fix: Option<Fix>,
    pub entrance: Option<Vec<Entrance>>,
    pub equate: Option<Equate>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fix {
    pub point: String,
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Equate {
    pub points: Vec<EqPoint>
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EqPoint {
    pub from: String,
    pub to: String,
}
pub type Entrance = String;
// Have to keep this and the reuturn statements otherwise error
#[allow(clippy::needless_return)]
pub fn load_config(config_path: String, v: bool) -> Option<Config> {
    if v {
        println!("Loading config: {:?}", config_path);
    }
    let path_buf = PathBuf::from(config_path);
    if path_buf.exists() && path_buf.extension().is_some_and(|n| n == "yaml") {
        println!("Config found: {}", path_buf.as_os_str().to_str().unwrap());
        let config = match std::fs::read_to_string(path_buf) {
            Ok(d) => self::parse(d),
            Err(_) => {
                println!("Couln't open config, using defaults");
                return None
            }
        };
        return config.ok();
    } else {
        println!("Config not found!");
        return None
    }
}
/// Print config
pub fn print_config(config: &Config) {
    dbg!(config);
}   
pub fn write_config(config: &Config, path: &mut PathBuf) -> Result<(), SurvexError>{
    let name = config.project.name.clone().to_lowercase().replace(" ", "-");
    path.pop();
    let config_path = path.join(format!("{}.yaml", name));
    let s = serde_yaml::to_string(config).unwrap();
    println!("{:?}", config_path);
    std::fs::write(config_path, s)?;
    Ok(())
}
pub fn parse(input: String) -> Result<Config, ()> {
    println!("Parsing config");
    let s = input.as_str();
    let t: Config = match serde_yaml::from_str(s) {
        Ok(a) => a,
        Err(e) => {
            println!("Error parsing config, using defaults. Error: {}", e);
            self::load_default()
        }
    };
    Ok(t)
}
pub fn load_default() -> Config {
    let s = r#"
    project:
     name: hello
     author: world
    "#;
    let a: Config = serde_yaml::from_str(s).unwrap();
    a
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for Fix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.point, self.lat, self.lon, self.elevation)
    }
}
impl Display for EqPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.from, self.to)
    }
}
