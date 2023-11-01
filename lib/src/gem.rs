use crate::gem_color::GemColor;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct Gem {
    pub color: GemColor,
}

impl Gem {
    pub fn new(color: GemColor) -> Gem {
        Gem { color }
    }
    pub fn random() -> Gem {
        Gem::new(GemColor::random())
    }
}

impl Display for Gem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl Default for Gem {
    fn default() -> Self {
        Gem::random()
    }
}
