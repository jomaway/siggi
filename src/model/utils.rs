use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Color { 
    Yellow,
    Blue, 
    Red,
    White,
    Black,
    Darkgray,
    Lightgray,
    Custom((u8,u8,u8)),
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Yellow => write!(f,"#FFC300"),
            Color::Blue => write!(f,"#0D84F0"),
            Color::Red => write!(f,"#FF5733"),
            Color::White => write!(f,"#FFFFFF"),
            Color::Black => write!(f,"#000000"),
            Color::Darkgray => write!(f,"#1c2833"),
            Color::Lightgray => write!(f,"#74838f"),
            Color::Custom((r,g,b)) => write!(f,"#{:#2x}{:#2x}{:#2x}",r,g,b),
        }
    }
}