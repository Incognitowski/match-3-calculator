use std::time::Instant;
use rand::Rng;

#[derive(Debug, PartialEq, Clone)]
enum GemColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
}

#[derive(Debug, PartialEq, Clone)]
struct Gem {
    color: GemColor,
    y: u8,
    x: u8,
}

impl ToString for Gem {
    fn to_string(&self) -> String {
        match self.color {
            GemColor::Blue => "🟦",
            GemColor::Green => "🟩",
            GemColor::Red => "🟥",
            GemColor::Yellow => "🟨",
            GemColor::Purple => "🟪",
        }.to_string()
    }
}

const BOARD_LENGTH: i32 = 8;
const BOARD_HEIGHT: i32 = 8;
const MIN_MATCH_AMOUNT: i32 = 3;
const MAX_MATCH_AMOUNT: i32 = 8;

fn get_random_color() -> GemColor {
    let rng = rand::thread_rng().gen_range(0..5);
    match rng {
        0 => GemColor::Blue,
        1 => GemColor::Green,
        2 => GemColor::Red,
        3 => GemColor::Yellow,
        _ => GemColor::Purple,
    }
}

fn build_random_gem(x: u8, y: u8) -> Gem {
    Gem {
        color: get_random_color(),
        y: x,
        x: y,
    }
}

fn print_board(board: &[Vec<Gem>]) {
    for (i, row) in board.iter().enumerate() {
        print!("{}", i);
        for gem in row {
            print!("{}", gem.to_string())
        }
        println!()
    }
}

fn check_for_combinations(board: &[Vec<Gem>]) {
    let transposed_board = transpose_board(board);
    let mut matches: Vec<&[Gem]> = vec![];
    let mut horizontal_matches = get_row_matches(board);
    let mut vertical_matches = get_row_matches(transposed_board.as_slice());
    matches.append(&mut horizontal_matches);
    matches.append(&mut vertical_matches);
    print_found_matches(&matches);
    let mut sanitized_matches: Vec<&[Gem]> = vec![];
    for gem_match in matches {
        if !already_contains_bigger_combination(gem_match, &sanitized_matches) {
            sanitized_matches.push(gem_match);
        }
    }
    if !sanitized_matches.is_empty() {
        println!("Valid, sanitized matches: {}", sanitized_matches.len())
    } else {
        println!("There no are matches.")
    }
}

fn print_found_matches(matches : &[&[Gem]]) {
    if matches.is_empty() {
        return;
    }
    println!("Combinations (Pre Sanitization)");
    for gem_match in matches {
        print!("{} =>", gem_match.first().unwrap().to_string());
        for it in 0..gem_match.len() {
            let gem = gem_match.get(it).unwrap();
            print!("({},{})", gem.y, gem.x);
        }
        println!();
    }
}

fn get_row_matches(board: &[Vec<Gem>]) -> Vec<&[Gem]> {
    let mut matches: Vec<&[Gem]> = vec![];
    let cloned_board = board.clone();
    for row_num in 0..cloned_board.len() {
        let row = cloned_board.get(row_num).unwrap();
        for match_amount in (MIN_MATCH_AMOUNT..(MAX_MATCH_AMOUNT+1)).rev() {
            for skip in 0..((i32::try_from(row.len()).unwrap() - match_amount)+1) {
                let sub_list = get_sub_list(skip, skip + match_amount, row);
                if gems_are_all_identical(sub_list) {
                    matches.push(sub_list);
                }
            }
        }
    }
    matches
}

fn transpose_board(board: &[Vec<Gem>]) -> Vec<Vec<Gem>> {
    let mut new_board: Vec<Vec<Gem>> = vec![];
    for col_num in 0..BOARD_LENGTH {
        let mut new_row: Vec<Gem> = vec![];
        for row in board {
            let gem = row.get(usize::try_from(col_num).unwrap()).unwrap().clone();
            new_row.push(gem);
        }
        new_board.push(new_row);
    }
    new_board
}

fn get_sub_list(from: i32, to: i32, target: &[Gem]) -> &[Gem] {
    let from_usize = usize::try_from(from).unwrap();
    let to_usize = usize::try_from(to).unwrap();
    target[from_usize..to_usize].as_ref()
}

fn already_contains_bigger_combination(comb: &[Gem], combs: &[&[Gem]]) -> bool {
    combs.iter().any(|ref_comb|
        ref_comb.windows(comb.len()).any(|window| comb == window)
    )
}

fn gems_are_all_identical(gems: &[Gem]) -> bool {
    let ref_gem = gems.first().unwrap();
    gems.iter().all(|gem| gem.color == ref_gem.color)
}


fn main() {
    let mut board: Vec<Vec<Gem>> = vec![];
    for row_num in 0..BOARD_HEIGHT {
        let mut row: Vec<Gem> = vec![];
        for column_num in 0..BOARD_LENGTH {
            let gem = build_random_gem(
                u8::try_from(row_num).unwrap(),
                u8::try_from(column_num).unwrap(),
            );
            row.push(gem);
        }
        board.push(row);
    }
    println!("Normal board:");
    print_board(&board);
    println!("Transposed board:");
    let transposed_board = transpose_board(&board);
    print_board(&transposed_board);
    println!("Combinations:");
    let started_at = Instant::now();
    check_for_combinations(&board);
    let elapsed = started_at.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
