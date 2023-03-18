#![feature(generators)]
#![feature(iter_from_generator)]


extern crate derive_more;
// use the derives that you want in the file

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, derive_more::Add, derive_more::AddAssign, derive_more::Sub,
)]
pub struct Point {
    x: i32,
    y: i32,
}

const SPAWN_POINT: Point = Point { x: 500, y: 0 };

// Note: X+ goes right, Y+ goes down
#[wasm_bindgen]
pub struct Grid {
    origin: Point,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    settled: usize,
    current_grain: Point,
}


#[wasm_bindgen]
impl Grid {
    // omitted: other methods

    #[wasm_bindgen]
    pub fn num_settled(&self) -> usize {
        self.settled
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        let straight_down = self.current_grain + Point { x: 0, y: 1 };
        let down_left = self.current_grain + Point { x: -1, y: 1 };
        let down_right = self.current_grain + Point { x: 1, y: 1 };
        let options = [straight_down, down_left, down_right];

        // Can we move?
        if let Some(pos) = options
            .into_iter()
            .find(|pos| matches!(self.cell(*pos), Some(Cell::Air)))
        {
            self.current_grain = pos;
            return;
        }

        // If not, are we moving off-screen?
        if let Some(pos) = options.into_iter().find(|pos| self.cell(*pos).is_none()) {
            // yes, we're done
            return;
        }

        // If not, then we've settled
        self.settled += 1;
        *self.cell_mut(self.current_grain).unwrap() = Cell::Sand;
        self.current_grain = SPAWN_POINT;
    }
}


impl Point {
    fn parse(s: &str) -> Self {
		// yolo error handling, you'll excuse me, we have other things
		// to get to.
        let mut tokens = s.split(',');
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Point {
	// that one's not in a trait
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::parse(include_str!("../input.txt"))
    }

    #[wasm_bindgen]
    pub fn render(&self, canvas_id: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(self.width as _);
        canvas.set_height(self.height as _);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let cell = self.cell(point).unwrap();
                let color = match cell {
                    Cell::Air => "#4db4e3",
                    Cell::Rock => "#33302d",
                    Cell::Sand => "#827f58",
                };
                context.set_fill_style(&JsValue::from_str(color));
                context.fill_rect(x as _, y as _, 1.0, 1.0);
            }
        }
    }
}

#[derive(Debug)]
pub struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    pub fn parse(s: &str) -> Self {
        Self {
            points: s.split(" -> ").map(Point::parse).collect(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Rock,
    Sand,
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let polylines: Vec<_> = input.lines().map(Polyline::parse).collect();

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        // sand falls from `(500,0)`
        let sand_spawn = Point { x: 500, y: 0 };

        for point in polylines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(std::iter::once(&sand_spawn))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        dbg!(min_x, max_x);
        dbg!(min_y, max_y);
        let origin = Point { x: min_x, y: min_y };
        let width: usize = (max_x - min_x + 1).try_into().unwrap();
        let height: usize = (max_y - min_y + 1).try_into().unwrap();
        dbg!(origin, width, height);
        let mut grid = Self {
            origin,
            width,
            height,
            cells: vec![Cell::Air; width * height],
            settled: todo!(),
            current_grain: todo!(),
        };

        for point in polylines.iter().flat_map(|p| p.path_points()) {
            *grid.cell_mut(point).unwrap() = Cell::Rock;
        }
        grid
    }

    fn cell_index(&self, point: Point) -> Option<usize> {
        let Point { x, y } = point - self.origin;
        // negative coords after offsetting = outside of grid
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn cell(&self, point: Point) -> Option<&Cell> {
        Some(&self.cells[self.cell_index(point)?])
    }

    fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
        // borrow checker won't let us do that inline 🙃
        let cell_index = self.cell_index(point)?;
        Some(&mut self.cells[cell_index])
    }
}


impl Polyline {
    pub fn path_points(&self) -> impl Iterator<Item = Point> + '_ {
        std::iter::from_generator(|| {
            let mut points = self.points.iter().copied();
            let Some(mut a) = points.next() else { return };
            yield a;

            loop {
                let Some(b) = points.next() else { return };
                let delta = (b - a).signum();
                assert!((delta.x == 0) ^ (delta.y == 0));

                loop {
                    a += delta;
                    yield a;
                    if a == b {
                        break;
                    }
                }
            }
        })
    }
}