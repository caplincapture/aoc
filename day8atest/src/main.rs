use grid::Grid;

use crate::grid::GridCoord;

mod grid;

fn visible_trees_in_dir(grid: &Grid<usize>, coord: GridCoord, (dx, dy): (isize, isize)) -> usize {
    let line = (1..).into_iter().map_while(|i| {
        let coord = GridCoord {
            x: coord.x.checked_add_signed(dx * i)?,
            y: coord.y.checked_add_signed(dy * i)?,
        };
        Some(*grid.cell(coord)?)
    });

    let mut total = 0;
    let our_height = *grid.cell(coord).unwrap();
    for height in line {
        total += 1;
        if height >= our_height {
            break;
        }
    }
    total
}

fn scenic_score(grid: &Grid<usize>, coord: GridCoord) -> usize {
    let dirs: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    dirs.into_iter()
        .map(|(dx, dy)| visible_trees_in_dir(grid, coord, (dx, dy)))
        .product()
}

fn main() {
    let grid = parse_grid(include_str!("input.txt"));

    let all_coords = (0..grid.height())
        .into_iter()
        .flat_map(|y| (0..grid.width()).map(move |x| GridCoord::from((x, y))));

    let best_place = all_coords
        .map(|coord| (coord, scenic_score(&grid, coord)))
        .max_by_key(|(_, score)| *score)
        .unwrap();

    println!("{best_place:?}");
}

fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }

    grid
}