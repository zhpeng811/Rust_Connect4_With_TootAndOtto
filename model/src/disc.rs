use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiscType {
    Empty,
    Red,
    Yellow,
    T,
    O,
}

impl Display for DiscType {
    fn fmt(&self, w: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            DiscType::Red => write!(w, "R"),
            DiscType::Yellow => write!(w, "Y"),
            DiscType::T => write!(w, "T"),
            DiscType::O => write!(w, "O"),
            DiscType::Empty => write!(w, "E"),
        }
    }
}
