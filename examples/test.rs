use siggi::{compose::Compositor, model::{Diagram, utils::Color, signal::{Clock, SignalGenerator, Wave}, Signal, marker, Lane}};


fn main() {
    // Create an Compositor
    let compositor = Compositor::default();

    let doc = compositor.compose(&generate_test_diagram());
    svg::save("test.svg", &doc).expect("Could not save the diagram.");
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

    let markerline = marker::Marker::new(1.5,true,1.2, Color::Red);
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

    
    d1.set_xaxis("time");

    d1.add(l3)
}
