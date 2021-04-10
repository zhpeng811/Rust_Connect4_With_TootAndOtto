pub use crate::disc::DiscType;

#[derive(Debug, PartialEq)]
pub enum PlayerType {
    Human,
    AI
}

#[derive(Debug, PartialEq)]
pub struct Player {
    pub player_type: PlayerType,
    pub disc_type: DiscType
}

impl Player {
    pub fn new(player_type: PlayerType, disc_type: DiscType) -> Self {
        Self {
            player_type,
            disc_type
        }
    }

    pub fn change_disc_type(&mut self, new_type: DiscType) {
        self.disc_type = new_type;
    }
}
