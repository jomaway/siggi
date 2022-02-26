use siggi::{compose::Compositor, model::{Diagram, utils::Color, signal::{Clock, SignalGenerator}, Signal, markers::{Label, Line}, Lane}};


fn main() {
    // Create an Compositor
    let compositor = Compositor::default();

    let doc = compositor.compose(&generate_manchester_diagram());
    svg::save("manchester.svg", &doc).expect("Could not save the diagram.");
}


fn generate_manchester_diagram() -> Diagram {
    let clk = Clock::negativ(11);
    let clk = clk.to_signal().color_with(Color::Yellow);

    let s1= Signal::new("Data", "hlhllhhhllh".parse().unwrap()).color_with(Color::Red);
    let s2 = Signal::new("Manchester","ududduuuddu".parse().unwrap()).color_with(Color::Blue);

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

    let mut diagram = Diagram::new(Some("Manchester Encoding".to_string()));

    let lane1 = Lane::new(clk).add_marker(markerline).clone();
    let mut lane2 = Lane::new(s1).add_marker(markerline).clone();
    let mut lane3 = Lane::new(s2).add_marker(markerline).clone();
        
    lane2.labels.extend(labels.clone());
    lane3.labels.extend(labels);

    diagram.append(lane1);
    diagram.append(lane2);
    diagram.append(lane3);

    diagram
}