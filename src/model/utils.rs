use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color { 
    Yellow,
    Blue, 
    Red,
    White,
    Black,
    Darkgray,
    Lightgray,
    Custom(&'static str),
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
            Color::Custom(string) => write!(f,"{}",string),
        }
    }
}

impl Color {
    pub fn as_html_color_str(& self) -> &str {
        match self {
            Color::Yellow => "#FFC300",
            Color::Blue => "#0D84F0",
            Color::Red => "#FF5733",
            Color::White => "#FFFFFF",
            Color::Black => "#000000",
            Color::Darkgray => "#1c2833",
            Color::Lightgray => "#74838f",
            Color::Custom(string) => string,
        }
    }
}