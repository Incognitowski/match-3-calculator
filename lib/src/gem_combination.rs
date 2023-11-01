use crate::constants::MIN_COMB_SIZE;
use crate::gem::Gem;
use crate::gem_color::GemColor;
use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq)]
pub struct GemCombination {
    pub color: GemColor,
    pub combinations: Vec<(usize, usize)>,
}

impl GemCombination {
    pub fn new(color: &GemColor) -> Self {
        Self {
            color: *color,
            combinations: vec![],
        }
    }
    pub fn new_with_combinations(color: &GemColor, combinations: Vec<(usize, usize)>) -> Self {
        Self {
            color: *color,
            combinations,
        }
    }
    pub fn from_match(
        gem_match: &[Gem],
        row: usize,
        starting_column: usize,
        invert_coordinates: bool,
    ) -> Self {
        let color = gem_match.first().unwrap().color;
        let mut combination = GemCombination::new(&color);
        for col_num in 0..MIN_COMB_SIZE {
            if invert_coordinates {
                combination
                    .combinations
                    .push((row, starting_column + col_num))
            } else {
                combination
                    .combinations
                    .push((starting_column + col_num, row))
            }
        }
        combination
    }
}

impl Display for GemCombination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut vec_str = String::new();
        for (x, y) in &self.combinations {
            vec_str.push_str(&format!("({}, {})", x, y))
        }
        write!(f, "Color: {} | Matches: {}", self.color, vec_str)
    }
}
