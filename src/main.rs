use init_with::InitWith;
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::time::Instant;

const BOARD_SIZE: i32 = 8;

const MIN_MATCH_AMOUNT: i32 = 3;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum GemColor {
    Blue,
    Green,
    Red,
    Yellow,
    Purple,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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
struct Gem {
    color: GemColor,
    y: i32,
    x: i32,
    bottom: Option<Rc<RefCell<Gem>>>,
    top: Option<Rc<RefCell<Gem>>>,
    left: Option<Rc<RefCell<Gem>>>,
    right: Option<Rc<RefCell<Gem>>>,
}

impl Gem {
    fn new(x: i32, y: i32, color: GemColor) -> Gem {
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
    fn new_random(x: i32, y: i32) -> Gem {
        Gem::new(x, y, GemColor::get_random_color())
    }
}

struct Board {
    board: [[Rc<RefCell<Gem>>; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

impl Board {
    fn new() -> Board {
        let new_board = Board {
            board:
                <[[Rc<RefCell<Gem>>; BOARD_SIZE as usize]; BOARD_SIZE as usize]>::init_with_indices(
                    |r| {
                        <[Rc<RefCell<Gem>>; BOARD_SIZE as usize]>::init_with_indices(|c| {
                            Rc::new(RefCell::new(Gem::new_random(
                                i32::try_from(c).unwrap(),
                                i32::try_from(r).unwrap(),
                            )))
                        })
                    },
                ),
        };

        for row_num in 0..BOARD_SIZE {
            let row_pos = usize::try_from(row_num).unwrap();
            for col_num in 0..BOARD_SIZE {
                let col_pos = usize::try_from(col_num).unwrap();
                let mut target_gem = new_board.board[row_pos][col_pos].borrow_mut();
                let top_gem = if row_num - 1 > 0 {
                    Some(new_board.board[row_pos - 1][col_pos].clone())
                } else {
                    None
                };
                let right_gem = if col_num + 1 < BOARD_SIZE {
                    Some(new_board.board[row_pos][col_pos + 1].clone())
                } else {
                    None
                };
                let bottom_gem = if row_num + 1 < BOARD_SIZE {
                    Some(new_board.board[row_pos + 1][col_pos].clone())
                } else {
                    None
                };
                let left_gem = if col_num - 1 > 0 {
                    Some(new_board.board[row_pos][col_pos - 1].clone())
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

#[derive(PartialEq, Eq, Hash)]
struct GemCoord {
    pub gem_color: GemColor,
    pub x: i32,
    pub y: i32,
}

impl GemCoord {
    fn new(x: i32, y: i32, color: GemColor) -> GemCoord {
        GemCoord {
            gem_color: color,
            x,
            y,
        }
    }
}

fn check_for_combinations(board: &Board) {
    let mut combination_finder = CombinationFinder::new(board);
    combination_finder.check_for_combinations();
    combination_finder.print_combinations();
}

struct CombinationFinder<'r> {
    board: &'r Board,
    gems_checked: HashSet<(Direction, GemCoord)>,
    found_combinations: Vec<Vec<GemCoord>>,
    has_possible_combination: bool,
}

impl<'r> CombinationFinder<'r> {
    fn new(board: &'r Board) -> CombinationFinder {
        CombinationFinder {
            board,
            gems_checked: HashSet::new(),
            found_combinations: vec![],
            has_possible_combination: false,
        }
    }

    fn check_for_combinations(&mut self) {
        for gem_row in &self.board.board {
            for gem_ref in gem_row {
                let gem = gem_ref.borrow();
                let mut combinations: Vec<Vec<&Gem>> = vec![];
                let up_combinations = self.check_for_combination(&gem, 0, Direction::Up);
                if !up_combinations.is_empty() {
                    combinations.push(up_combinations);
                }
                let right_combinations = self.check_for_combination(&gem, 0, Direction::Right);
                if !right_combinations.is_empty() {
                    combinations.push(right_combinations);
                }
                let down_combinations = self.check_for_combination(&gem, 0, Direction::Down);
                if !down_combinations.is_empty() {
                    combinations.push(down_combinations);
                }
                let left_combinations = self.check_for_combination(&gem, 0, Direction::Left);
                if !left_combinations.is_empty() {
                    combinations.push(left_combinations);
                }
                let mut combinations: Vec<Vec<GemCoord>> = combinations
                    .iter()
                    .filter(|&i| !i.is_empty())
                    .map(|i| {
                        i.iter()
                            .map(|&gem| GemCoord::new(gem.x, gem.y, gem.color.clone()))
                            .collect()
                    })
                    .collect();
                self.found_combinations.append(&mut combinations);
            }
        }
    }

    fn print_combinations(&self) {}

    fn check_for_combination(&mut self, gem: &Gem, depth: i32, direction: Direction) -> Vec<&Gem> {
        let hash_comb = (direction, GemCoord::new(gem.x, gem.y, gem.color.clone()));
        if self.gems_checked.contains(&hash_comb) {
            return vec![];
        }
        self.gems_checked.insert((direction, hash_comb.1));
        let mut result: Vec<&Gem> = vec![];
        let next_gem = match direction {
            Direction::Up => &gem.top,
            Direction::Right => &gem.right,
            Direction::Down => &gem.bottom,
            Direction::Left => &gem.left,
        };
        if next_gem.is_none() {
            return vec![];
        }
        let next_gem = next_gem.as_ref().unwrap();
        let next_gem = next_gem.borrow();
        if &gem.color == &next_gem.color {
            let mut depth_check = self.check_for_combination(&next_gem, depth + 1, direction);
            result.append(&mut depth_check);
        } else {
            self.check_for_surrounding_gems(&next_gem, &gem.color, direction);
        }
        result
    }

    fn check_for_surrounding_gems(
        &self,
        gem: &Gem,
        color: &GemColor,
        original_direction: Direction,
    ) {
        if original_direction != Direction::Up {
            match &gem.top {
                None => {}
                Some(gem) => { if &gem.as_ref().borrow().color == color {

                } }
            }
        }
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
    check_for_combinations(&board);
    let elapsed = started_at.elapsed();
    println!("Time to check for combinations: {:.2?}", elapsed);
}
