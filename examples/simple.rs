use siggi::{model::Diagram, compose::Compositor};

fn main() {
    // Create an Compositor
    let compositor = Compositor::default();

    let diagram = Diagram::new(Some("title".to_string()));

    let doc = compositor.compose(&diagram);
    svg::save("manchester.svg", &doc).expect("Could not save the diagram.");
}