
use std::{fs};

use siggi::{compose::Compositor, parse};

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
 struct Args {
    #[clap(short, long)]
    signal: Vec<String>,

    #[clap(short, long)]
    clock: Vec<String>,

    #[clap(short, long)]
    #[clap(default_value_t = String::from("Siggi Diagram"))]
    title: String,

    #[clap(short, long)]
    #[clap(default_value_t = String::from("./target/siggi.svg"))]
    output: String,

    #[clap(short,long)]
    dark: bool,

    #[clap(long)]
    ticks: bool,

    #[clap(long)]
    tocks: bool,

    #[clap(short,long)]
    input_file: Option<String>
}

fn main() {
    println!("Starting, siggi!");

    let args = Args::parse();
    let comp = Compositor::default();

    if let Some(path) = args.input_file {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let diagram = parse::from_json_str(&data).expect("Error while parsing signals");
        let doc = comp.compose(&diagram);
        svg::save(args.output, &doc).expect("Could not save the diagram.");
    } else {
        let diagram = parse::from_args(args.title, args.dark, args.clock, args.signal).expect("Parsing error");
        let doc = comp.compose(&diagram);
        svg::save(args.output, &doc).expect("Could not save the diagram.");
    }
}

