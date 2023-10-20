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

impl ToString for GemColor {
    fn to_string(&self) -> String {
        match self {
            GemColor::Blue => "🟦",
            GemColor::Green => "🟩",
            GemColor::Red => "🟥",
            GemColor::Yellow => "🟨",
            GemColor::Purple => "🟪",
        }.to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Gem<'r> {
    color: GemColor,
    y: i32,
    x: i32,
    bottom: Option<&'r Gem<'r>>,
    top: Option<&'r Gem<'r>>,
    left: Option<&'r Gem<'r>>,
    right: Option<&'r Gem<'r>>,
}

#[derive(Debug, PartialEq, Clone)]
struct GemAddress<'r> {
    x: i32,
    y: i32,
    gem: Gem<'r>,
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
    fn new_referenced(x: i32, y: i32, color: GemColor, boundaries: (Option<&'r Gem>, Option<&'r Gem>, Option<&'r Gem>, Option<&'r Gem>)) -> Gem<'r> {
        Gem {
            color,
            y,
            x,
            top: boundaries.0,
            right: boundaries.1,
            bottom: boundaries.2,
            left: boundaries.3,
        }
    }
}

impl<'r> GemAddress<'r> {
    fn new(x: i32, y: i32, gem: Gem<'r>) -> GemAddress<'r> {
        GemAddress {
            x,
            y,
            gem,
        }
    }
}

trait FindByCoords<'r> {
    fn find_by_address(&'r self, x: i32, y: i32) -> Option<&'r Gem<'r>>;
}

impl<'r> FindByCoords<'r> for Vec<GemAddress<'r>> {
    fn find_by_address(&'r self, x: i32, y: i32) -> Option<&'r Gem<'r>> {
        let gem_address = self.iter().find(|i| i.x == x && i.y == y);
        if let Some(gem) = gem_address {
            Some(&gem.gem)
        } else {
            None
        }
    }
}

const BOARD_SIZE: i32 = 8;
const MIN_MATCH_AMOUNT: i32 = 3;

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

fn get_random_gem<'r>(x: i32, y: i32) -> Gem<'r> {
    Gem::new(x, y, get_random_color())
}

fn generate_board<'r>() -> Vec<GemAddress<'r>> {
    let mut gem_addresses: Vec<GemAddress> = vec![];

    for row_num in 0..BOARD_SIZE {
        for col_num in 0..BOARD_SIZE {
            gem_addresses.push(
                GemAddress::new(
                    row_num,
                    col_num,
                    get_random_gem(row_num, col_num),
                )
            );
        }
    }

    let mut referenced_gem_addresses: Vec<GemAddress> = vec![];

    for row_num in 0..BOARD_SIZE {
        for col_num in 0..BOARD_SIZE {
            let target_gem = gem_addresses.find_by_address(row_num, col_num).unwrap().to_owned();
            let top_gem = gem_addresses.find_by_address(row_num + 1, col_num);
            let right_gem = gem_addresses.find_by_address(row_num, col_num + 1);
            let bottom_gem = gem_addresses.find_by_address(row_num - 1, col_num);
            let left_gem = gem_addresses.find_by_address(row_num, col_num - 1);
            referenced_gem_addresses.push(
                GemAddress::new(
                    row_num,
                    col_num,
                    Gem::new_referenced(row_num, col_num, target_gem.color, (top_gem, right_gem, bottom_gem, left_gem)),
                )
            );
        }
    }

    // cannot return value referencing local variable `gem_addresses`
    // returns a value referencing data owned by the current function
    referenced_gem_addresses
}

fn main() {
    let started_at = Instant::now();
    let board = generate_board();
    let elapsed = started_at.elapsed();
    println!("Time to generate board: {:.2?}", elapsed);
    let started_at = Instant::now();
    print_board(&board);
    let elapsed = started_at.elapsed();
    println!("Time to print board: {:.2?}", elapsed);
    // check_for_combinations(&board);
}

fn print_board(board: &Vec<GemAddress>) {
    let mut col_ref = board.find_by_address(0, 0).unwrap();
    while col_ref.bottom.is_some() {
        let mut row_ref = col_ref;
        while row_ref.right.is_some() {
            println!("{}", row_ref.color.to_string());
            row_ref = row_ref.right.unwrap();
        }
        col_ref = col_ref.bottom.unwrap();
    }
}
