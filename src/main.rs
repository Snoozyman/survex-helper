extern crate libmnemo as mnemo;
extern crate libsurvex as survex;

use clap::Parser;
use mnemo::NemoReaderOptions;
use survex::types::{PointName, SurvexProject};
use survex::{mnemo::FromNemo, SurvexWriter};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Survex helper program to transform Mnemo files to Survex format
struct Args {
    /// Select path for reading a file or directory
    #[arg(short, long)]
    path: Option<String>,

    /// Optional, output filename, defaults to out.svx
    #[arg(short, long)]
    output: Option<String>,

    /// Print only. Prints information to stdout
    #[arg(long)]
    print: bool,

    /// File prefix for mnemo files
    #[arg(short, long, default_value = "MNEMO")]
    name: String,
}

fn main() {
    let args = Args::parse();
    
    // #[cfg(feature = "debug")]
    let path = match &args.path{
        None => "./",
        Some(p) => p,
    };
    if args.print && !path.is_empty() {
        print(path);
    }else {
        let output = match args.output {
            Some(o) => o,
            None => "out.svx".into(),
        };
        handle_file(path, output);
    }
}

fn handle_file(path: &str, output: String) {
    let options = NemoReaderOptions::default();
    let reader = Box::new(mnemo::NemoReader::new(path, options).unwrap());
    // dbg!(&reader);
    let p = SurvexProject::from_nemo(&reader);
    
    let wr = SurvexWriter::from(p);
    let o = output.clone();
    match wr.write_to_path(o) {
        Ok(_) => println!("Success! Written to: {}", output),
        Err(e) => eprintln!("Error: \n{}", e)
    }
}
fn print(path: &str) {
    let options = NemoReaderOptions::default();
    let reader = Box::new(mnemo::NemoReader::new(path, options).unwrap());
    println!("Reader: {}", &reader);
}