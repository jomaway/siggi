
use siggi::{model::{Signal, signal::{Wave, Clock, SignalGenerator, Level}, Diagram, Lane, markers::{Label, Line}, utils::Color}, compose::Compositor};

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

}

#[allow(unused)]
fn generate_test_diagram() -> Diagram {
    let nclk = Clock::negativ(4);
    let pclk = Clock::positiv(4).name("tclk");
    let nclk = nclk.to_signal();

    let mut s1= Signal::default();
    s1.name("ts 1").wave(Wave{ levels: vec![Level::Low,Level::High,Level::Low,Level::High]}).phase(0).period(1).color(Color::Yellow);

    let mut d1 = Diagram::new("Simple siggi diagram");

    d1.add(Lane::new(nclk).add_label(Label::default()).clone());
    d1.add(Lane::new(s1).add_marker(Line::default()).clone());

    let mut l3 = Lane::new(pclk.to_signal());
    l3.label_at(String::from("lab"), 2.3);
    l3.mark_at(1.8);
    //d1.add(l3);
    d1
}