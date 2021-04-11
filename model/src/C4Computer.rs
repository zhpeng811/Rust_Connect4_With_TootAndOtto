#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl ToString for Difficulty {
    fn to_string(&self) -> String {
        match self {
            Easy => String::from("Easy"),
            Medium => String::from("Medium"),
            Hard => String::from("Hard"),
        }
    }
}