use std::fmt::{Debug, Display};

use super::utils::Color;


// todo!() rename Line to Marker and trait Marker to Positionable

pub trait Marker {
    // Get the marker's position
    fn position(&self) -> f64;
    fn at(&mut self, position: f64) -> &mut Self;
}

impl Marker for Line {
    // Get the line's position
    fn position(&self) -> f64 {
        self.position
    }
    fn at(&mut self, position: f64) -> &mut Self {
        self.position = position;
        self
    }
}

impl Marker for Label {
    // Get the label's position
    fn position(&self) -> f64 {
        self.position
    }
    fn at(&mut self, position: f64) -> &mut Self {
        self.position = position;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    position: f64,
    dashed: bool,
    thickness: f64,
    color: Color,
}

impl Default for Line {
    fn default() -> Self {
        Self { position: 0.0, dashed: true, thickness: 1.0, color: Default::default() }
    }
}

impl Line {
    pub fn new(position: f64, dashed: bool, thickness: f64, color: Color) -> Self { Self { position, dashed, thickness, color } }
    
    fn at(mut self, position: f64) -> Self {
        self.position = position;
        self
    }
}



#[derive(Debug, Clone, Copy, Default)]
pub struct Label{
    text: &'static str,
    position: f64,
    color: Color,
    size: TextSize,
    anchor: TextAnchor,
}

impl Label {
    pub fn new(text: &'static str) -> Self { Self { text, ..Default::default()} }

    pub fn small(text: &'static str) -> Self {
        Self { text, size: TextSize::Small, color: Color::Lightgray, ..Default::default()}
    }

    pub fn large(text: &'static str) -> Self {
        Self { text, size: TextSize::Large, ..Default::default()}
    }

    fn at(mut self, position: f64) -> Self {
        self.position = position;
        self
    }

    /// Get the label's text.
    pub fn text(&self) -> &str {
        self.text
    }

    /// Get the label's color.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Get the label's size.
    pub fn size(&self) -> TextSize {
        self.size
    }

    /// Get the label's anchor.
    pub fn anchor(&self) -> TextAnchor {
        self.anchor
    }

    /// Set the label's color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Set the label's size.
    pub fn set_size(&mut self, size: TextSize) {
        self.size = size;
    }

    /// Set the label's anchor.
    pub fn set_anchor(&mut self, anchor: TextAnchor) {
        self.anchor = anchor;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextSize {
    Small,
    Smaller,
    Normal,
    Larger,
    Large,
}

impl Default for TextSize {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for TextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextSize::Small => write!(f, "small"),
            TextSize::Smaller => write!(f, "smaller"),
            TextSize::Normal => write!(f, "normal"),
            TextSize::Larger => write!(f, "larger"),
            TextSize::Large => write!(f, "large"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TextAnchor {
    Start,
    End,
    Middle,
}

impl Default for TextAnchor {
    fn default() -> Self {
        Self::Middle
    }
}


impl Display for TextAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TextAnchor::Start => write!(f,"start"),
            TextAnchor::End => write!(f,"end"),
            TextAnchor::Middle => write!(f,"middle"),
        }
    }
}