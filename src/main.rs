use init_with::{InitWith};
use rand::Rng;
use std::cell::RefCell;
use std::time::Instant;

const BOARD_SIZE: i32 = 8;

const MIN_MATCH_AMOUNT: i32 = 3;

#[derive(Debug, PartialEq, Clone)]
enum GemColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
}

impl GemColor {
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
}

impl ToString for GemColor {
    fn to_string(&self) -> String {
        match self {
            GemColor::Blue => "🟦",
            GemColor::Green => "🟩",
            GemColor::Red => "🟥",
            GemColor::Yellow => "🟨",
            GemColor::Purple => "🟪",
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Gem<'r> {
    color: GemColor,
    y: i32,
    x: i32,
    bottom: Option<&'r RefCell<Gem<'r>>>,
    top: Option<&'r RefCell<Gem<'r>>>,
    left: Option<&'r RefCell<Gem<'r>>>,
    right: Option<&'r RefCell<Gem<'r>>>,
}

impl<'r> Gem<'r> {
    fn new(x: i32, y: i32, color: GemColor) -> Gem<'r> {
        Gem {
            color,
            y,
            x,
            bottom: None,
            top: None,
            left: None,
            right: None,
        }
    }
    fn new_random(x: i32, y: i32) -> Gem<'r> {
        Gem::new(x, y, GemColor::get_random_color())
    }
}

#[derive(Clone)]
struct Board<'r> {
    board: [[RefCell<Gem<'r>>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

impl<'r> Board<'r> {
    fn new() -> Board<'r> {
        let new_board = Board {
            board:
                <[[RefCell<Gem<'r>>; BOARD_SIZE as usize]; BOARD_SIZE as usize]>::init_with_indices(
                    |r| {
                        <[RefCell<Gem<'r>>; BOARD_SIZE as usize]>::init_with_indices(|c| {
                            RefCell::new(Gem::new_random(
                                i32::try_from(r).unwrap(),
                                i32::try_from(c).unwrap(),
                            ))
                        })
                    },
                ),
        };

        for row_num in 0..BOARD_SIZE {
            let row_pos = usize::try_from(row_num).unwrap();
            for col_num in 0..BOARD_SIZE {
                let col_pos = usize::try_from(col_num).unwrap();
                let mut target_gem = new_board.board[row_pos][col_pos].borrow_mut();
                let top_gem = if row_pos + 1 < BOARD_SIZE as usize {
                    Some(&new_board.board[row_pos + 1][col_pos])
                } else {
                    None
                };
                let right_gem = if col_pos + 1 < BOARD_SIZE as usize {
                    Some(&new_board.board[row_pos][col_pos + 1])
                } else {
                    None
                };
                let bottom_gem = if row_pos - 1 > 0 {
                    Some(&new_board.board[row_pos - 1][col_pos])
                } else {
                    None
                };
                let left_gem = if col_pos - 1 > 0 {
                    Some(&new_board.board[row_pos][col_pos - 1])
                } else {
                    None
                };
                target_gem.top = top_gem;
                target_gem.right = right_gem;
                target_gem.bottom = bottom_gem;
                target_gem.left = left_gem;
            }
        }

        new_board
    }

    fn print(&self) {
        for row in &self.board {
            for gem_ref in row {
                let gem = gem_ref.borrow();
                print!("{}", gem.color.to_string())
            }
            println!()
        }
    }
}

fn main() {
    let started_at = Instant::now();
    // let board = generate_board();
    let board = Board::new();
    let elapsed = started_at.elapsed();
    println!("Time to generate board: {:.2?}", elapsed);
    let started_at = Instant::now();
    board.print();
    // print_board(&board);
    let elapsed = started_at.elapsed();
    println!("Time to print board: {:.2?}", elapsed);
    // check_for_combinations(&board);
}