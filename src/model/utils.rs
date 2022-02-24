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