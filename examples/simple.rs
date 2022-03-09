use siggi::{model::Diagram, compose::Compositor};

fn main() {
    // Create an Compositor
    let compositor = Compositor::default();

    let mut diagram = Diagram::new(Some("title".to_string()));
    diagram.set_xaxis("x axis label");

    let doc = compositor.compose(&diagram);
    svg::save("simple.svg", &doc).expect("Could not save the diagram.");
}