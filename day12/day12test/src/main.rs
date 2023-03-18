use day12test::Grid;

fn main() {
    let grid = Grid::parse(include_str!("input.txt"));
    println!("{grid:?}");
}