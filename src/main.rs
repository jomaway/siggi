
use siggi::{model::{Signal, signal::{Wave, Clock, SignalGenerator, Level}, Diagram, Lane, markers::{Label, Line}, utils::Color}, compose::Compositor};

fn main() {
    println!("Hello, siggi!");

    let nclk = Clock::negativ(4);
    let pclk = Clock::positiv(4).name("tclk");
    let nclk = nclk.to_signal();

    let mut s1= Signal::default();
    s1.name("ts 1").wave(Wave{ levels: vec![Level::Low,Level::High,Level::Low,Level::High]}).phase(0).period(1).color(Color::Yellow);

    let mut d1 = Diagram::new("Simple siggi diagram");

    d1.add(Lane::new(nclk).add_label(Label::default()).clone());
    d1.add(Lane::new(s1).add_marker(Line::default()).clone());

    let mut l3 = Lane::new(pclk.to_signal());
    l3.label_at("lab", 2.3);
    l3.mark_at(1.8);
    //d1.add(l3);

    let comp = Compositor::default();
    let doc = comp.compose(&d1);

    svg::save("target/siggi.svg", &doc).expect("Could not save the diagram.");

}
