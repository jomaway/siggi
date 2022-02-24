use std::fmt::Debug;

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

    fn at(mut self, position: f64) -> Self {
        self.position = position;
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