use crate::board::Board;
use crate::constants::BOARD_SIZE;
use crate::gem::Gem;
use crate::gem_color::GemColor;
use init_with::InitWith;

pub fn board_from_string(board_as_string: &str) -> Board {
    let rows: Vec<&str> = board_as_string
        .split('\n')
        .map(|row| row.trim())
        .filter(|row| !row.is_empty())
        .collect();
    Board::from(<[[Gem; BOARD_SIZE]; BOARD_SIZE]>::init_with_indices(|x| {
        let row: &str = rows.get(x).unwrap();
        let row: Vec<char> = row.chars().collect();
        <[Gem; BOARD_SIZE]>::init_with_indices(|y| Gem::new(GemColor::from_char(row[y])))
    }))
}
