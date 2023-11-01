use rand::Rng;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum GemColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
}

impl GemColor {
    pub fn random() -> GemColor {
        let rng = rand::thread_rng().gen_range(0..5);
        match rng {
            0 => GemColor::Blue,
            1 => GemColor::Green,
            2 => GemColor::Red,
            3 => GemColor::Yellow,
            _ => GemColor::Purple,
        }
    }

    pub fn from_char(gem_char: char) -> GemColor {
        match gem_char {
            '🟦' => GemColor::Blue,
            '🟩' => GemColor::Green,
            '🟥' => GemColor::Red,
            '🟨' => GemColor::Yellow,
            '🟪' => GemColor::Purple,
            _ => panic!("{} was given instead of valid color char.", gem_char),
        }
    }
}

impl Display for GemColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GemColor::Blue => "🟦",
                GemColor::Green => "🟩",
                GemColor::Red => "🟥",
                GemColor::Yellow => "🟨",
                GemColor::Purple => "🟪",
            }
        )
    }
}
