use crate::gem::Gem;
use crate::gem_color::GemColor;
use crate::gem_combination::GemCombination;
use init_with::InitWith;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::time::Instant;

mod gem;
mod gem_color;
mod gem_combination;

const BOARD_SIZE: usize = 8;
const BOARD_SIZE_I: i32 = 8;
const MIN_COMB_SIZE: i32 = 3;
const MIN_COMB_SIZE_U: usize = 3;
const POSSIBLE_COMB_SIZE: i32 = MIN_COMB_SIZE - 1;

struct Board {
    board: [[Gem; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn new() -> Self {
        Self {
            board: <[[Gem; BOARD_SIZE]; BOARD_SIZE]>::init_with_indices(|_| {
                <[Gem; BOARD_SIZE]>::init_with_indices(|_| Gem::random())
            }),
        }
    }

    fn from(board: [[Gem; BOARD_SIZE]; BOARD_SIZE]) -> Self {
        Self { board }
    }

    fn transpose(&self) -> Board {
        let mut transposed_board = [[Gem::default(); BOARD_SIZE]; BOARD_SIZE];
        for (row_num, row) in self.board.iter().enumerate() {
            for (col_num, gem) in row.iter().enumerate() {
                transposed_board[col_num][row_num] = *gem;
            }
        }
        Board::from(transposed_board)
    }

    fn print(&self) {
        for row in &self.board {
            for gem in row {
                print!("{}", gem)
            }
            println!()
        }
    }
}

fn gems_are_matching(possible_match: &[Gem]) -> bool {
    let color = possible_match.first().unwrap().color;
    possible_match.iter().all(|gem| gem.color == color)
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

fn find_combinations(board: &Board, skip_transposition: bool) -> Vec<GemCombination> {
    let mut result: Vec<GemCombination> = vec![];
    for (row_num, row) in board.board.iter().enumerate() {
        for (starting_col, possible_comb) in row.windows(MIN_COMB_SIZE as usize).enumerate() {
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

fn combinations_should_be_merged(a: &GemCombination, b: &GemCombination) -> bool {
    // if a.color != b.color {
    //     return false;
    // }
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

fn print_found_combinations(combinations: &Vec<GemCombination>) {
    for gem_combination in combinations {
        println!("{}", gem_combination)
    }
}

fn main() {
    let started_at = Instant::now();
    let board = Board::new();
    let elapsed = started_at.elapsed();
    println!("Time to generate board: {:.2?}", elapsed);

    let started_at = Instant::now();
    board.print();
    let elapsed = started_at.elapsed();
    println!("Time to print board: {:.2?}", elapsed);

    let started_at = Instant::now();
    let combinations = find_combinations(&board, false);
    let elapsed = started_at.elapsed();
    println!("Time to find combinations: {:.2?}", elapsed);

    print_found_combinations(&combinations);

    if contains_mergeable_combinations(&combinations) {
        println!("Contains mergeable combinations.")
    }

    // println!("----------------------------");
    //
    // let started_at = Instant::now();
    // let board = board.transpose();
    // let elapsed = started_at.elapsed();
    // println!("Time to transpose board: {:.2?}", elapsed);
    //
    // let started_at = Instant::now();
    // board.print();
    // let elapsed = started_at.elapsed();
    // println!("Time to print board: {:.2?}", elapsed);
}
