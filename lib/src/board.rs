use crate::constants::{BOARD_SIZE, MIN_COMB_SIZE, POSSIBLE_COMB_SIZE};
use crate::gem::Gem;
use crate::gem_color::GemColor;
use crate::gem_combination::GemCombination;
use init_with::InitWith;
use std::collections::HashSet;

pub struct Board {
    pub board: [[Gem; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn new() -> Self {
        Self {
            board: <[[Gem; BOARD_SIZE]; BOARD_SIZE]>::init_with_indices(|_| {
                <[Gem; BOARD_SIZE]>::init_with_indices(|_| Gem::random())
            }),
        }
    }

    pub fn from(board: [[Gem; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self { board }
    }

    pub fn transpose(&self) -> Board {
        let mut transposed_board = [[Gem::default(); BOARD_SIZE]; BOARD_SIZE];
        for (row_num, row) in self.board.iter().enumerate() {
            for (col_num, gem) in row.iter().enumerate() {
                transposed_board[col_num][row_num] = *gem;
            }
        }
        Board::from(transposed_board)
    }

    pub fn print(&self) {
        for row in &self.board {
            for gem in row {
                print!("{}", gem)
            }
            println!()
        }
    }

    pub fn contains_possible_combinations(&self) -> bool {
        contains_possible_combinations(self) || contains_possible_combinations(&self.transpose())
    }

    pub fn find_combinations(&self) -> Vec<GemCombination> {
        find_combinations(self, false)
    }
}

fn contains_possible_combinations(board: &Board) -> bool {
    for (row_num, row) in board.board.iter().enumerate() {
        for (iter, gems) in row.windows(POSSIBLE_COMB_SIZE).enumerate() {
            if !gems_are_matching(gems) {
                continue;
            }
            let target_color = &gems.first().unwrap().color;
            let mut target_gems: Vec<&Gem> = vec![];
            if iter > 0 {
                if row_num > 0 {
                    target_gems.push(&board.board[row_num - 1][iter - 1]);
                }
                if row_num < BOARD_SIZE - 1 {
                    target_gems.push(&board.board[row_num + 1][iter - 1]);
                }
            }
            if iter > 1 {
                target_gems.push(&board.board[row_num][iter - 2]);
            }
            if iter < BOARD_SIZE - 3 {
                if row_num > 0 {
                    target_gems.push(&board.board[row_num - 1][iter + 2]);
                }
                if row_num < BOARD_SIZE - 1 {
                    target_gems.push(&board.board[row_num + 1][iter + 2]);
                }
            }
            if iter < BOARD_SIZE - 4 {
                target_gems.push(&board.board[row_num][iter + 3]);
            }
            let color_occurrence = count_color_occurrence(&target_gems, target_color);
            if color_occurrence > 0 {
                return true;
            }
        }
    }
    for (row_num, row) in board.board.iter().enumerate() {
        for (iter, gems) in row.windows(MIN_COMB_SIZE).enumerate() {
            let mut target_gems: Vec<&Gem> = vec![];
            if row_num > 0 {
                target_gems.push(&board.board[row_num - 1][iter + 1])
            }
            if row_num < BOARD_SIZE - 2 {
                target_gems.push(&board.board[row_num + 1][iter + 1])
            }
            let target_color = &gems.first().unwrap().color;
            let color_occurrence = count_color_occurrence(&target_gems, target_color);
            if color_occurrence > 0 {
                return true;
            }
        }
    }
    false
}

fn gems_are_matching(possible_match: &[Gem]) -> bool {
    let color = possible_match.first().unwrap().color;
    possible_match.iter().all(|gem| gem.color == color)
}

fn count_color_occurrence(gems: &[&Gem], gem_color: &GemColor) -> i32 {
    let mut result = 0;
    for gem in gems {
        if &gem.color == gem_color {
            result += 1
        }
    }
    result
}

fn find_combinations(board: &Board, skip_transposition: bool) -> Vec<GemCombination> {
    let mut result: Vec<GemCombination> = vec![];
    for (row_num, row) in board.board.iter().enumerate() {
        for (starting_col, possible_comb) in row.windows(MIN_COMB_SIZE).enumerate() {
            if gems_are_matching(possible_comb) {
                result.push(GemCombination::from_match(
                    possible_comb,
                    row_num,
                    starting_col,
                    skip_transposition,
                ))
            }
        }
    }
    if !skip_transposition {
        let transposed_board = board.transpose();
        result.append(&mut find_combinations(&transposed_board, true))
    }
    while contains_mergeable_combinations(&result) {
        result = sanitize_combinations(result)
    }
    result
}

fn contains_mergeable_combinations(combinations: &Vec<GemCombination>) -> bool {
    let mut unique_array_size: usize = 0;
    for x in combinations {
        unique_array_size += x.combinations.len();
    }
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();
    for gem_combination in combinations {
        for (x, y) in &gem_combination.combinations {
            coordinates.insert((*x, *y));
        }
    }
    coordinates.len() < unique_array_size
}

fn combinations_should_be_merged(a: &GemCombination, b: &GemCombination) -> bool {
    if a.color != b.color {
        return false;
    }
    for a_coord in &a.combinations {
        for b_coord in &b.combinations {
            if a_coord == b_coord {
                return true;
            }
        }
    }
    false
}

fn merge_combinations(mut a: GemCombination, b: GemCombination) -> GemCombination {
    let mut new_combinations: Vec<(usize, usize)> = vec![];
    new_combinations.append(a.combinations.as_mut());
    for b_combination in &b.combinations {
        if !new_combinations.contains(b_combination) {
            new_combinations.push(*b_combination)
        }
    }
    GemCombination::new_with_combinations(&a.color, new_combinations)
}

fn sanitize_combinations(combinations: Vec<GemCombination>) -> Vec<GemCombination> {
    let mut result: Vec<GemCombination> = vec![];
    let mut ignore: Vec<&GemCombination> = vec![];
    for outer in &combinations {
        if ignore.contains(&outer) {
            continue;
        }
        for inner in &combinations {
            if ignore.contains(&inner) {
                continue;
            }
            if inner == outer {
                continue;
            }
            if combinations_should_be_merged(outer, inner) {
                result.push(merge_combinations(outer.clone(), inner.clone()));
                ignore.push(inner);
                ignore.push(outer);
            }
        }
    }
    for gem_combination in &combinations {
        if !ignore.contains(&gem_combination) {
            result.push(gem_combination.clone())
        }
    }
    result
}
