pub mod signal;
pub mod utils;
pub mod markers;

pub use signal::Signal;

use self::{utils::Color, markers::{Line, Label, Marker}};

#[derive(Debug, Default, Clone)]
pub struct Diagram {
    title: String,
    lanes: Vec<Lane>,
    background: Color,
}

impl Diagram {
    pub fn new<T>(title: T) -> Self where T: Into<String> { 
        Self { title: title.into(), background: Color::White, ..Default::default() } 
    }

    pub fn dark(&mut self) -> &mut Self {
        self.background = Color::Darkgray;
        self
    }

    // Add a lane to the diagram.
    pub fn add(&mut self, lane: Lane) {
        self.lanes.push(lane);
    }

    /// Set the diagram's background.
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    /// Get the diagram's title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the diagram's background.
    pub fn background(&self) -> Color {
        self.background
    }

    // Get the number of lanes in the diagram
    pub fn lane_count(&self) -> usize {
        self.lanes.len()
    }

    /// Get a reference to the diagram's lanes.
    pub fn lanes(&self) -> &[Lane] {
        self.lanes.as_ref()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Lane {
    pub sig: Signal,
    pub markers: Vec<Line>,
    pub labels: Vec<Label>,
}

impl Lane {
    pub fn new(sig: Signal) -> Self { Self { sig, ..Default::default()} }

    pub fn add_marker(&mut self, marker: Line) -> &mut Self {
        self.markers.push(marker);
        self
    }

    pub fn add_label(&mut self, label: Label) -> &mut Self {
        self.labels.push(label);
        self
    }

    pub fn mark_at(&mut self, position: f64) {
        self.markers.push(*Line::default().at(position));
    }

    pub fn label_at(&mut self, text: String, position: f64) {

        self.labels.push(Label::new(text).at(position).clone());
    }
}

