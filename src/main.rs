
use std::{str::FromStr, fs};

use siggi::{model::{Signal, signal::{Wave, Clock, SignalGenerator}, Diagram, Lane, markers::{Label, Line}, utils::Color}, compose::Compositor, parse::{self, }};

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

    #[clap(long)]
    test: bool,

    #[clap(short,long)]
    input_file: Option<String>
}

fn main() {
    println!("Starting, siggi!");

    let args = Args::parse();
    let comp = Compositor::default();

    if args.test {
        svg::save("target/test.svg",&comp.compose(&generate_test_diagram())).expect("Test failed");
    }

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
    // svg::save("target/test.svg", &comp.compose(&generate_manchester_example())).expect("Could not save the test diagram.");
}

#[allow(unused)]
fn generate_test_diagram() -> Diagram {
    let nclk = Clock::negativ(4);
    let pclk = Clock::positiv(4).name("tclk");
    let nclk = nclk.to_signal();

    let mut s1= Signal::default();
    s1.set_name("ts 1")
        .set_wave( "hlhlhlhlhlhlhlhlhlhl".parse::<Wave>().unwrap())
        .set_phase(0)
        .set_period(0.2)
        .set_color(Color::Yellow);

    let mut d1 = Diagram::new(Some("Simple siggi diagram".to_string()));

    let markerline = Line::new(1.5,true,1.2, Color::Red);
    d1.append(
        Lane::new(nclk)
            .add_marker(markerline)
            .add_label_at("rising edge", 1.5)
        );
    d1.append(
        Lane::new(s1)
            .add_marker(markerline)
            .add_label_at("0", 0.5)
            .add_label_at("1", 1.5)
            .add_label_at("0", 2.5)
            .add_label_at("1", 3.5)
        );

    let mut l3 = Lane::new(pclk.to_signal().color_with(Color::Blue));
    
    l3.append_label_at(String::from("&lt;- t -&gt;"), 1.5);
    l3.append_marker_at(1.0);
    l3.append_marker_at(2.0);

    d1.add(l3)
}

#[allow(unused)]
fn generate_manchester_example() -> Diagram {
    let clk = Clock::negativ(11);
    let clk = clk.to_signal().color_with(Color::Yellow);

    let mut s1= Signal::new("Data", Wave::from_str("hlhllhhhllh").unwrap() );
    s1.set_color(Color::Red);

    let mut s2 = Signal::new("Manchester",Wave::from_str("ududduuuddu").unwrap());
    s2.set_color(Color::Blue);

    let labels = vec![
        Label::from("1").at(0.5),
        Label::from("0").at(1.5),
        Label::from("1").at(2.5),
        Label::from("0").at(3.5),
        Label::from("0").at(4.5),
        Label::from("1").at(5.5),
        Label::from("1").at(6.5),
        Label::from("1").at(7.5),
        Label::from("0").at(8.5),
        Label::from("0").at(9.5),
        Label::from("0").at(10.5)
    ];

    let markerline = Line::new(1.5,true,1.2, Color::Red);

    let mut d1 = Diagram::new(Some("Manchester Encoding".to_string()));

    let l1 = Lane::new(clk).add_marker(markerline).clone();
    let mut l2 = Lane::new(s1).add_marker(markerline).clone();
    let mut l3 = Lane::new(s2).add_marker(markerline).clone();
        
    l2.labels.extend(labels.clone());
    l3.labels.extend(labels);

    d1.append(l1);
    d1.append(l2);
    d1.append(l3);

    d1
}