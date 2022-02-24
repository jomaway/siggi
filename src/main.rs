
use std::str::FromStr;

use siggi::{model::{Signal, signal::{Wave, Clock, SignalGenerator, Level}, Diagram, Lane, markers::{Label, Line, Marker}, utils::Color}, compose::Compositor};

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
    #[clap(default_value_t = String::from("siggi.svg"))]
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

    let mut diag = Diagram::new(args.title);
    if args.dark {
        diag.dark();
    }
    
    for clock in args.clock.iter() {
        diag.add(Lane::new(clock.parse::<Signal>().expect("Could not parse clock from given string")))
    }
    
    for (num,wave) in args.signal.iter().enumerate() {
        let wave = wave.parse::<Wave>().expect("Could not parse wave from given string.");
        let signal = Signal::new().wave(wave).name(format!("s-{}",num)).clone();
        
        diag.add(Lane::new(signal));
    }
    

    let comp = Compositor::default();
    let doc = comp.compose(&diag);

    svg::save("target/siggi.svg", &doc).expect("Could not save the diagram.");


    svg::save("target/test.svg", &comp.compose(&generate_manchester_example())).expect("Could not save the test diagram.");

}

#[allow(unused)]
fn generate_test_diagram() -> Diagram {
    let nclk = Clock::negativ(4);
    let pclk = Clock::positiv(4).name("tclk");
    let nclk = nclk.to_signal();

    let mut s1= Signal::default();
    s1.name("ts 1").wave(Wave{ levels: vec![Level::Low,Level::High,Level::Low,Level::High]}).phase(0).period(1).color(Color::Yellow);

    let mut d1 = Diagram::new("Simple siggi diagram");

    let markerline = Line::new(1.5,true,1.2, Color::Red);
    d1.add(Lane::new(nclk).add_marker(markerline)
        .with_label_at("rising edge", 1.5).clone());
    d1.add(Lane::new(s1).add_marker(markerline)
        .with_label_at("0", 0.5)
        .with_label_at("1", 1.5)
        .with_label_at("0", 2.5)
        .with_label_at("1", 3.5)
        .clone());

    let mut l3 = Lane::new(pclk.to_signal().color(Color::Blue).clone());
    l3.label_at(String::from("&lt;- t -&gt;"), 1.5);
    l3.mark_at(1.0);
    l3.mark_at(2.0);
    d1.add(l3);
    d1
}

#[allow(unused)]
fn generate_manchester_example() -> Diagram {
    let clk = Clock::negativ(11);
    let clk = clk.to_signal().color(Color::Yellow).clone();

    let mut s1= Signal::default();
    s1.name("Data").wave(Wave::from_str("hlhllhhhllh").unwrap()).color(Color::Red);

    let mut s2 = Signal::default();
    s2.name("Manchester").wave(Wave::from_str("ududduuuddu").unwrap()).color(Color::Blue);

    let labels = vec![
        Label::new("1").at(0.5).clone(),
        Label::new("0").at(1.5).clone(),
        Label::new("1").at(2.5).clone(),
        Label::new("0").at(3.5).clone(),
        Label::new("0").at(4.5).clone(),
        Label::new("1").at(5.5).clone(),
        Label::new("1").at(6.5).clone(),
        Label::new("1").at(7.5).clone(),
        Label::new("0").at(8.5).clone(),
        Label::new("0").at(9.5).clone(),
        Label::new("1").at(10.5).clone()
    ];

    let markerline = Line::new(1.5,true,1.2, Color::Red);

    let mut d1 = Diagram::new("Manchester Encoding");

    let l1 = Lane::new(clk).add_marker(markerline).clone();
    let mut l2 = Lane::new(s1).add_marker(markerline).clone();
    let mut l3 = Lane::new(s2).add_marker(markerline).clone();
        
    l2.labels.extend(labels.clone());
    l3.labels.extend(labels);

    d1.add(l1);
    d1.add(l2);
    d1.add(l3);

    d1
}