extern crate libmnemo as mnemo;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Optional, defaults to current directory
    #[arg(short, long, default_value="./")]
    path: String,

    #[arg(short, long)]
    test: bool,
    /// File input: true/false
    #[arg(short, long)]
    file: bool,

    /// Dir input: true/false
    #[arg(short, long)]
    dir: bool,

    /// File prefix for mnemo files
    #[arg(short, long, default_value="MNEMO")]
    name: String
}

fn main() {
    let args = Args::parse();
    #[cfg(feature="debug")]
    println!("{:?}", args);

    let path = args.path;
    if args.file {
        handle_file(path);
    } else if args.dir {
        handle_dir(path, args.name);
    } else if args.test {
    }
}

fn handle_file(path: String) {
    
    let file = mnemo::read_nemo_file(path);
    match file {
        Ok(f) => mnemo::test::debug_nemofile(f),
        Err(e) => e.print(),
    };
}
fn handle_dir(path: String, name: String) {

    let f = match mnemo::read_nemo_dir(path, name) { 
        Ok(files) => {
            for file in files {
                mnemo::test::debug_nemofile(file);
            }
        }
        Err(_) => println!("No files found")
    };
    
}
