use crate::gem_combination::GemCombination;

use crate::board::Board;
use std::time::Instant;

mod board;
mod constants;
mod gem;
mod gem_color;
mod gem_combination;
mod utilities;

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

    board.print();

    let started_at = Instant::now();
    let combinations = board.find_combinations();
    let elapsed = started_at.elapsed();
    println!("Time to find combinations: {:.2?}", elapsed);

    print_found_combinations(&combinations);

    let started_at = Instant::now();
    if board.contains_possible_combinations() {
        println!("Contains possible combinations.");
    } else {
        println!("Does not contain possible combinations.")
    }
    let elapsed = started_at.elapsed();
    println!("Time to find possible combinations: {:.2?}", elapsed);
}
