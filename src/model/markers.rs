use std::fmt::{Debug, Display};

use super::utils::Color;

// todo!() rename Line to Marker and trait Marker to Positionable

pub trait Marker {
    // Get the marker's position
    fn position(&self) -> f64;
}

impl Marker for Line {
    // Get the line's position
    fn position(&self) -> f64 {
        self.position
    }
}

impl Marker for Label {
    // Get the label's position
    fn position(&self) -> f64 {
        self.position
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub position: f64,
    pub dashed: bool,
    pub thickness: f64,
    pub color: Color,
}

impl Default for Line {
    fn default() -> Self {
        Self { position: 0.0, dashed: true, thickness: 1.0, color: Color::Lightgray }
    }
}

impl Line {
    pub fn new(position: f64, dashed: bool, thickness: f64, color: Color) -> Self { Self { position, dashed, thickness, color } }
    
    /// Places the Line at a given wave position
    /// Supports builder pattern.
    pub fn at(mut self, position: f64) -> Self {
        self.position = position;
        self
    }
}



#[derive(Debug, Clone, Default)]
pub struct Label{
    pub text: String,
    pub position: f64,
    pub color: Color,
    pub size: TextSize,
    pub anchor: TextAnchor,
}

impl From<&str> for Label { 
    fn from(text: &str) -> Self{
        Self {
            text: text.into(),
            ..Default::default()
        }
    }
}

impl From<String> for Label { 
    fn from(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

impl Label {

    pub fn at(mut self, position: f64) -> Self{
        self.position = position;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = TextSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = TextSize::Large;
        self
    }

    pub fn with_size(mut self, size: TextSize) -> Self {
        self.size = size;
        self
    }

    pub fn align(mut self, anchor: TextAnchor) ->  Self {
        self.anchor = anchor;
        self
    }

    pub fn color_with(mut self, color: Color) -> Self {
        self.color = color;
        self
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