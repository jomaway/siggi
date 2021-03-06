
use super::{utils::Color, marker::{Marker, Label, TextSize}, Signal};


#[derive(Debug, Clone)]
pub struct Diagram {
    title: Option<String>,
    lanes: Vec<Lane>,
    background: Color,
    xaxis: Option<String>,
}

impl Default for Diagram {
    fn default() -> Self {
        Self { title: Default::default(), lanes: Default::default(), background: Color::White, xaxis: None }
    }
}

impl Diagram {

    /// Constructs a new Diagram
    /// Sets title to None if the string is empty.
    pub fn new(title: Option<String>) -> Self { 
        if let Some(title) = &title {
            if title.is_empty() {
                return Self::default();
            }
        }
        Self { title, background: Color::White, ..Default::default() } 
    }

    /// Adds an xaxis arrow to the bottom of the diagram
    /// Supports builder pattern
    pub fn has_xaxis(mut self, xaxis: &str) -> Self {
        self.xaxis = Some(String::from(xaxis));
        self
    }

    /// Set the diagram's background to Darkgray if true.
    /// Supports builder pattern
    pub fn dark(mut self, dark: bool) -> Self {
        if dark {
            self.background = Color::Darkgray;
        }
        self
    }

    /// Add a lane to the diagram.
    /// Supports builder pattern
    pub fn add(mut self, lane: Lane) -> Self {
        self.lanes.push(lane);
        self
    }

    /// Append a lane to the diagram.
    pub fn append(&mut self, lane: Lane) -> &mut Self {
        self.lanes.push(lane);
        self
    }

    /// Set the diagram's title.
    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    /// Set the diagram's background.
    pub fn set_background(&mut self, background: Color) {
        self.background = background;
    }

    /// Set the diagram's xaxis.
    pub fn set_xaxis(&mut self, xaxis: &str) {
        self.xaxis = Some(String::from(xaxis));
    }

    /// Get a reference to the diagram's title.
    pub fn title(&self) -> String {
        self.title.as_ref().unwrap_or(&String::default()).clone()  // todo!() change clone() to return &String
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

    /// Get a reference to the diagram's xaxis.
    pub fn xaxis(&self) -> Option<&String> {
        self.xaxis.as_ref()
    }
}


#[derive(Debug, Default, Clone)]
pub struct Lane {
    pub signal: Signal,
    pub markers: Vec<Marker>,
    pub labels: Vec<Label>,
}

impl Lane {
    pub fn new(signal: Signal) -> Self { Self { signal, ..Default::default()} }

    /// Add marker to the lane
    /// Supports builder pattern
    pub fn add_marker(mut self, marker: Marker) -> Self {
        self.markers.push(marker);
        self
    }

    /// Add label to the lane
    /// Supports builder pattern
    pub fn add_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    /// Append maker to the lane
    pub fn append_marker(&mut self, marker: Marker) -> &mut Self {
        self.markers.push(marker);
        self
    }

    /// Append label to the lane
    pub fn append_label(&mut self, label: Label) -> &mut Self {
        self.labels.push(label);
        self
    }

    /// Add a default marker at the given position
    /// Supports builder pattern
    pub fn add_mark_at(mut self,  position: f64) -> Self  {
        self.append_marker_at(position);
        self
    }

    /// Append a default marker at the given position
    pub fn append_marker_at(&mut self, position: f64) -> &mut Self {
        self.markers.push(Marker::default().at(position));
        self
    }

    /// Add a text as default label at the given position 
    /// Supports builder pattern
    pub fn add_label_at<T>(mut self, text: T, position: f64) -> Self where T: Into<String> {
        self.append_label_at(text, position);
        self
    }

    /// Append a text as default label at the given position
    pub fn append_label_at<T>(&mut self, text: T, position: f64) -> &mut Self where T: Into<String> {
        self.labels.push(Label::from(text.into()).at(position).with_size(TextSize::Small));
        self
    }

}

