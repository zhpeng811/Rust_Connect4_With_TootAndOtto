
// common board sizes from wikipedia page: https://en.wikipedia.org/wiki/Connect_Four
// its in the form of "# columns" By "# of rows"
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BoardSize {
    SevenBySix,
    FiveByFour,
    SixByFive,
    EightBySeven,
    NineBySeven,
    TenBySeven,
    EightByEight,
}

impl ToString for BoardSize {
    fn to_string(&self) -> String {
        match self {
            BoardSize::SevenBySix => String::from("7 x 6"),
            BoardSize::FiveByFour => String::from("5 x 4"),
            BoardSize::SixByFive => String::from("6 x 5"),
            BoardSize::EightBySeven => String::from("8 x 7"),
            BoardSize::NineBySeven => String::from("9 x 7"),
            BoardSize::TenBySeven => String::from("10 x 7"),
            BoardSize::EightByEight => String::from("8 x 8"),
        }
    }
}

impl BoardSize {
    pub fn get_row(&self) -> usize {
        match self {
            BoardSize::SevenBySix => return 6,
            BoardSize::FiveByFour => return 4,
            BoardSize::SixByFive => return 5,
            BoardSize::EightBySeven => return 7,
            BoardSize::NineBySeven => return 7,
            BoardSize::TenBySeven => return 7,
            BoardSize::EightByEight => return 8,
        }
    }

    pub fn get_column(&self) -> usize {
        match self {
            BoardSize::SevenBySix => return 7,
            BoardSize::FiveByFour => return 5,
            BoardSize::SixByFive => return 6,
            BoardSize::EightBySeven => return 8,
            BoardSize::NineBySeven => return 9,
            BoardSize::TenBySeven => return 10,
            BoardSize::EightByEight => return 8,
        }
    }

    pub fn to_vec() -> Vec<BoardSize> {
        vec![
            BoardSize::SevenBySix, 
            BoardSize::FiveByFour,
            BoardSize::SixByFive,
            BoardSize::EightBySeven,
            BoardSize::NineBySeven,
            BoardSize::TenBySeven,
            BoardSize::EightByEight
        ]
    }
}