
use siggi::model::{Signal, signal::{Wave, Clock, SignalGenerator}, Diagram, Lane, markers::{Label, Line}};

fn main() {
    println!("Hello, world!");

    let s1 = Signal::default().name("Test").clone();
    let mut s2= Signal::default();
    s2.name("ts2").wave(Wave::default()).phase(0.2).period(1.5);

    let nclk = Clock::negativ(4);
    let pclk = Clock::positiv(4).name("tclk");

    let s3 = nclk.to_signal();

    println!("{:?}",s1);
    println!("{:?}",s2);
    println!("{:?}",s3);
    println!("{:?}",nclk);
    println!("{:?}",pclk); 

    let mut d1 = Diagram::new("title");

    d1.add(Lane::new(s1).add_label(Label::default()).clone());
    d1.add(Lane::new(s2).add_marker(Line::default()).clone());
    d1.add(Lane::new(s3).add_label(Label::default()).add_marker(Line::default()).clone());
    let mut l4 = Lane::new(pclk.to_signal());
    l4.label_at("lab", 2.3);
    l4.mark_at(1.8);
    d1.add(l4);

    dbg!(d1);
}
