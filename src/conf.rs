use serde::{Deserialize, Serialize};
use std::{fmt::{Display, Formatter}, path::PathBuf};
use toml::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub project: Project,
    pub options: Option<Options>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub author: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Options {}
// Have to keep this and the reuturn statements otherwise error
#[allow(clippy::needless_return)]
pub fn load_config(config_path: Option<&str>, v: bool) -> Config {
    if v {
        println!("Loading config: {:?}", config_path);
    }
    match config_path {
        Some(v) => {
            let path_buf = PathBuf::from(v);
            if path_buf.exists() && path_buf.extension().is_some_and(|n| n == "toml") {
                println!("Config found: {}", path_buf.as_os_str().to_str().unwrap());
                let config = match std::fs::read_to_string(v) {
                    Ok(d) => self::parse(d),
                    Err(_) => {
                        println!("Couln't open config, using defaults");
                        return load_default();
                    }
                };
                return config.unwrap();
            } else {
                println!("Config not found!");
                return load_default();
            }
        }
        None => return load_default(),
    };
}
pub fn parse(input: String) -> Result<Config, ()> {
    let s = input.as_str();
    let t: Config = match toml::from_str(s) {
        Ok(a) => a,
        Err(_) => self::load_default(),
    };
    Ok(t)
}
pub fn load_default() -> Config {
    let s = r#"
    [project]
    name = 'hello'
    author = 'world'
    "#;
    let a: Config = toml::from_str(s).unwrap();
    a
}
impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub fn write_config(config: &Config, path: &mut PathBuf){
    let name = config.project.name.clone().to_lowercase().replace(" ", "-");
    path.pop();
    let config_path = path.join(format!("{}-config.toml", name));
    let s = toml::to_string(config).unwrap();
    println!("{:?}", config_path);
    std::fs::write(config_path, s).unwrap();
}