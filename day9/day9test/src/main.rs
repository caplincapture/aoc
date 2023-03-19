use std::{collections::VecDeque, time::Duration};

use egui::{Color32, Sense, Slider, Stroke};
use nom::{combinator::all_consuming, Finish};
use parse::{Direction, GridPos, Instruction};

mod parse;

use eframe::{egui, epaint::ahash::HashSet};

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas",
            web_options,
            Box::new(|_cc| Box::new(MyApp::new())),
        )
        .await
        .expect("failed to start eframe");
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 — Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    instructions: VecDeque<Instruction>,
    knots: [GridPos; 10],
    tail_visited: HashSet<GridPos>,
    speed: usize,
    paused: bool,
    show_sidebar: bool,
    step: bool,
}

impl MyApp {
    fn new() -> Self {
        let instructions = include_str!("input.txt")
            .lines()
            .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
            .collect();

        Self {
            instructions,
            knots: [GridPos { x: 0, y: 0 }; 10],
            tail_visited: Default::default(),
            speed: 1,
            paused: true,
            show_sidebar: true,
            step: false,
        }
    }

    fn update_state(&mut self) {
        // I'd use "let-else" but it breaks rustfmt for now
        let instruction = match self.instructions.front_mut() {
            Some(instruction) => instruction,
            None => return,
        };
        self.knots[0] += instruction.dir.delta();

        for i in 1..self.knots.len() {
            let diff = self.knots[i - 1] - self.knots[i];
            let (dx, dy) = match (diff.x, diff.y) {
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                // 🆕 need to move diagonally
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("unhandled case: tail - head = {diff:?}"),
            };
            self.knots[i].x += dx;
            self.knots[i].y += dy;

            if i == self.knots.len() - 1 {
                self.tail_visited.insert(self.knots[i]);
            }
        }

        instruction.dist -= 1;
        if instruction.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.interact_size.y *= 1.4;
                ui.style_mut()
                    .text_styles
                    .get_mut(&egui::TextStyle::Button)
                    .unwrap()
                    .size *= 1.4;

                if ui.button("Reset").clicked() {
                    *self = Self::new();
                }
                if ui.button("Step").clicked() {
                    self.step = true;
                }

                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });

                ui.toggle_value(&mut self.show_sidebar, "Sidebar");
            });

            ui.horizontal(|ui| {
                ui.label("Speed: ");
                ui.add(Slider::new(&mut self.speed, 1..=20).prefix("x"));
            });
        });

        if self.step {
            self.update_state();
            self.step = false;
        } else if !self.paused {
            for _ in 0..self.speed {
                self.update_state();
            }
            ctx.request_repaint_after(Duration::from_millis(25));
        }

        if self.show_sidebar {
            egui::SidePanel::right("side_panel").show(ctx, |ui| {
                ui.label(format!("{} places visited", self.tail_visited.len()));
                egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    let mut it = self.instructions.iter();
                    for (i, ins) in it.by_ref().enumerate() {
                        if i >= 20 {
                            break;
                        }

                        let arrow = match ins.dir {
                            Direction::Up => "⬆",
                            Direction::Down => "⬇",
                            Direction::Right => "➡",
                            Direction::Left => "⬅",
                        };
                        let dist = ins.dist as usize;
                        if dist > 5 {
                            ui.label(format!("{}+{}", arrow.repeat(5), dist - 5));
                        } else {
                            ui.label(arrow.repeat(dist));
                        }
                    }
                    let remaining = it.count();

                    if remaining > 0 {
                        ui.label(format!("(+ {remaining} more)"));
                    }
                })
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }

            const SIDE: f32 = 5.0;

            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            let to_panel_pos = |pos: GridPos| {
                (egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE) + center).to_pos2()
            };

            let half_width = (painter_size.x / SIDE).floor() as i32;
            let half_height = (painter_size.y / SIDE).floor() as i32;

            for x in -half_width..half_width {
                for y in -half_height..half_height {
                    let dot = GridPos { x, y };
                    let color = if dot.x == 0 && dot.y == 0 {
                        Color32::WHITE
                    } else if self.tail_visited.contains(&dot) {
                        Color32::DARK_RED
                    } else {
                        continue;
                    };

                    let dot_pos = to_panel_pos(dot);
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }

            // paint the head
            let head_pos = to_panel_pos(self.head);
            painter.circle_stroke(head_pos, 2.0, Stroke::new(2.0, Color32::GREEN));

            // paint the tail
            let tail_pos = to_panel_pos(self.tail);
            painter.circle_stroke(tail_pos, 2.0, Stroke::new(2.0, Color32::YELLOW));

            // paint an arrow from head to tail
            painter.arrow(
                tail_pos,
                head_pos - tail_pos,
                Stroke::new(1.0, Color32::YELLOW),
            )
        });
    }
}