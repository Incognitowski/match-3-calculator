use petgraph::dot::Dot;
use petgraph::matrix_graph::MatrixGraph;
use petgraph::matrix_graph::NodeIndex;
use rand::Rng;
use std::fmt::{Debug, Display, Formatter};
use std::time::Instant;

const BOARD_SIZE: usize = 8;
const BOARD_SIZE_I: i32 = 8;

#[derive(PartialEq, Clone, Debug)]
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

impl Display for GemColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GemColor::Blue => "🟦",
                GemColor::Green => "🟩",
                GemColor::Red => "🟥",
                GemColor::Yellow => "🟨",
                GemColor::Purple => "🟪",
            }
        )
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Gem {
    color: GemColor,
}

impl Gem {
    fn new(color: GemColor) -> Gem {
        Gem { color }
    }
    fn new_random() -> Gem {
        Gem::new(GemColor::get_random_color())
    }
}

impl Display for Gem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.color)
    }
}

impl Default for Gem {
    fn default() -> Self {
        Gem::new_random()
    }
}

fn main() {
    let started_at = Instant::now();
    let mut graph = MatrixGraph::<Gem, i32>::new();
    let mut nodes: Vec<Vec<NodeIndex>> = vec![];
    for _ in 0..BOARD_SIZE {
        let mut row_nodes: Vec<NodeIndex> = vec![];
        for _ in 0..BOARD_SIZE {
            row_nodes.push(graph.add_node(Gem::new_random()));
        }
        nodes.push(row_nodes);
    }
    for (row_num, row) in nodes.iter().enumerate() {
        for (col_num, _) in row.iter().enumerate() {
            let row = i32::try_from(row_num).unwrap();
            let col = i32::try_from(col_num).unwrap();
            let node = nodes.get(row_num).unwrap().get(col_num).unwrap().to_owned();
            if (0..BOARD_SIZE_I - 1).contains(&row) {
                graph.add_edge(
                    node,
                    nodes
                        .get(row_num + 1)
                        .unwrap()
                        .get(col_num)
                        .unwrap()
                        .to_owned(),
                    1,
                )
            }
            if (1..BOARD_SIZE_I).contains(&row) {
                graph.add_edge(
                    node,
                    nodes
                        .get(row_num - 1)
                        .unwrap()
                        .get(col_num)
                        .unwrap()
                        .to_owned(),
                    1,
                )
            }
            if (0..BOARD_SIZE_I - 1).contains(&col) {
                graph.add_edge(
                    node,
                    nodes
                        .get(row_num)
                        .unwrap()
                        .get(col_num + 1)
                        .unwrap()
                        .to_owned(),
                    1,
                )
            }
            if (1..BOARD_SIZE_I).contains(&col) {
                graph.add_edge(
                    node,
                    nodes
                        .get(row_num)
                        .unwrap()
                        .get(col_num - 1)
                        .unwrap()
                        .to_owned(),
                    1,
                )
            }
        }
    }
    println!("{:?}", Dot::new(&graph));
    let elapsed = started_at.elapsed();
    println!("Time to generate board: {:.2?}", elapsed);
}
