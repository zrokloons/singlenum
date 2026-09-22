//! Optional native front end. The worker owns its puzzle copy and never touches UI state.
use eframe::egui;
use singlenum::solver::{solve, Puzzle, SolveResult, SolveStatus};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::Duration;

fn main() -> eframe::Result {
    env_logger::init();
    eframe::run_native(
        "Singlenum",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([560.0, 700.0])
                .with_min_inner_size([380.0, 520.0]),
            ..Default::default()
        },
        Box::new(|_| Ok(Box::new(SinglenumApp::default()))),
    )
}

struct SinglenumApp {
    path: String,
    loaded_path: String,
    puzzle: Option<Puzzle>,
    solution: Option<[usize; 81]>,
    show_original: bool,
    attempts: i32,
    status: String,
    is_error: bool,
    worker: Option<Receiver<anyhow::Result<SolveResult>>>,
}

impl Default for SinglenumApp {
    fn default() -> Self {
        Self {
            path: String::new(),
            loaded_path: String::new(),
            puzzle: None,
            solution: None,
            show_original: false,
            attempts: 500,
            status: "Enter a JSON puzzle path and choose Load.".into(),
            is_error: false,
            worker: None,
        }
    }
}

impl SinglenumApp {
    fn load(&mut self) {
        self.load_path(std::path::PathBuf::from(&self.path));
    }

    fn load_path(&mut self, path: std::path::PathBuf) {
        match Puzzle::load(&path) {
            Ok(puzzle) => {
                self.puzzle = Some(puzzle);
                self.loaded_path = path.display().to_string();
                self.reset();
                self.status = "Loaded. Ready to solve.".into();
            }
            Err(error) => {
                // A failed load must not discard the previously loaded puzzle.
                self.status = format!("{error:#}");
                self.is_error = true;
            }
        }
    }

    fn reset(&mut self) {
        self.solution = None;
        self.show_original = false;
        self.status = "Reset to the original puzzle. Ready to solve.".into();
        self.is_error = false;
    }

    fn start_solve(&mut self, context: &egui::Context) {
        let Some(puzzle) = self.puzzle.clone() else {
            return;
        };
        if self.worker.is_some() {
            return;
        }
        let attempts = self.attempts;
        let context = context.clone();
        let (sender, receiver) = mpsc::channel();
        let spawn = std::thread::Builder::new()
            .name("singlenum-solver".into())
            .spawn(move || {
                let _ = sender.send(solve(&puzzle, attempts));
                context.request_repaint();
            });
        match spawn {
            Ok(_) => {
                self.solution = None;
                self.show_original = false;
                self.worker = Some(receiver);
                self.status = "Solving…".into();
                self.is_error = false;
            }
            Err(error) => {
                self.status = format!("Cannot start solver: {error}");
                self.is_error = true;
            }
        }
    }

    fn poll_worker(&mut self) {
        let Some(worker) = &self.worker else { return };
        match worker.try_recv() {
            Ok(result) => {
                self.worker = None;
                match result {
                    Ok(result) if result.status == SolveStatus::Solved => {
                        self.solution =
                            Some(std::array::from_fn(|i| result.table.squares[i].value));
                        self.status = format!("Solved {}", result.message);
                        self.is_error = false;
                    }
                    Ok(result) => {
                        self.status = format!(
                            "Attempt limit reached {}. Try a larger limit. This does not prove there is no solution.",
                            result.message
                        );
                        self.is_error = false;
                    }
                    Err(error) => {
                        self.status = format!("Solver error: {error:#}");
                        self.is_error = true;
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                self.worker = None;
                self.status =
                    "The solver worker stopped unexpectedly. You can reset or retry.".into();
                self.is_error = true;
            }
        }
    }

    fn displayed_values(&self) -> Option<&[usize; 81]> {
        let puzzle = self.puzzle.as_ref()?;
        if self.show_original {
            Some(puzzle.values())
        } else {
            Some(self.solution.as_ref().unwrap_or(puzzle.values()))
        }
    }

    fn board(&self, ui: &mut egui::Ui) {
        let empty = [0; 81];
        let values = self.displayed_values().unwrap_or(&empty);
        let givens = self.puzzle.as_ref().map(Puzzle::values).unwrap_or(&empty);
        let side = ui.available_width().min(468.0);
        let cell = side / 9.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::hover());
        let painter = ui.painter();
        let ink = ui.visuals().text_color();
        let answer_ink = ui.visuals().hyperlink_color;
        for index in 0..81 {
            let min = rect.min + egui::vec2((index % 9) as f32 * cell, (index / 9) as f32 * cell);
            let cell_rect = egui::Rect::from_min_size(min, egui::vec2(cell, cell));
            if givens[index] != 0 {
                painter.rect_filled(cell_rect, 0.0, ui.visuals().widgets.inactive.bg_fill);
            }
            if values[index] != 0 {
                painter.text(
                    cell_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    values[index].to_string(),
                    egui::FontId::proportional(cell * 0.55),
                    if givens[index] != 0 { ink } else { answer_ink },
                );
            }
        }
        for line in 0..=9 {
            let offset = line as f32 * cell;
            let stroke = egui::Stroke::new(if line % 3 == 0 { 2.5_f32 } else { 0.7_f32 }, ink);
            painter.line_segment(
                [
                    rect.min + egui::vec2(offset, 0.0),
                    rect.min + egui::vec2(offset, side),
                ],
                stroke,
            );
            painter.line_segment(
                [
                    rect.min + egui::vec2(0.0, offset),
                    rect.min + egui::vec2(side, offset),
                ],
                stroke,
            );
        }
    }
}

impl eframe::App for SinglenumApp {
    fn update(&mut self, context: &egui::Context, _: &mut eframe::Frame) {
        self.poll_worker();
        let busy = self.worker.is_some();
        if busy {
            // Also poll for disconnection if a worker panics before requesting repaint.
            context.request_repaint_after(Duration::from_millis(100));
        }
        egui::CentralPanel::default().show(context, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Singlenum");
                ui.label("Load a Sudoku JSON file, then solve.");
                ui.add_enabled_ui(!busy, |ui| {
                    if ui.button("Open puzzle…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Sudoku JSON", &["json"])
                            .pick_file()
                        {
                            self.path = path.display().to_string();
                            self.load_path(path);
                        }
                    }
                    ui.horizontal(|ui| {
                        ui.label("JSON path:");
                        let field = ui.add(
                            egui::TextEdit::singleline(&mut self.path)
                                .hint_text("puzzles/cat/medium/puzzle_aa.json")
                                .desired_width((ui.available_width() - 60.0).max(100.0)),
                        );
                        let enter =
                            field.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                        if ui.button("Load").clicked() || enter {
                            self.load();
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Attempt limit:");
                        ui.add(egui::DragValue::new(&mut self.attempts).range(1..=100_000));
                        if ui
                            .add_enabled(self.puzzle.is_some(), egui::Button::new("Solve"))
                            .clicked()
                        {
                            self.start_solve(context);
                        }
                        // Recheck after Solve, which can start a worker in this frame.
                        if ui
                            .add_enabled(
                                self.puzzle.is_some() && self.worker.is_none(),
                                egui::Button::new("Reset"),
                            )
                            .clicked()
                        {
                            self.reset();
                        }
                    });
                });
                ui.add_enabled(
                    self.solution.is_some(),
                    egui::Checkbox::new(&mut self.show_original, "Show original"),
                );
                if self.puzzle.is_some() {
                    ui.label(format!("Loaded: {}", self.loaded_path));
                }
                if self.worker.is_some() {
                    ui.spinner();
                }
                if self.is_error {
                    ui.colored_label(ui.visuals().error_fg_color, &self.status);
                } else {
                    ui.label(&self.status);
                }
                ui.separator();
                self.board(ui);
                ui.label("Shaded cells: original clues. Colored digits: solved values.");
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_reset_and_original_toggle() {
        let mut app = SinglenumApp {
            path: "puzzles/cat/medium/puzzle_aa.json".into(),
            ..Default::default()
        };
        app.load();
        assert!(!app.is_error);
        let original = *app.puzzle.as_ref().unwrap().values();
        app.solution = Some([9; 81]);
        assert_eq!(app.displayed_values(), Some(&[9; 81]));
        app.show_original = true;
        assert_eq!(app.displayed_values(), Some(&original));
        app.reset();
        assert!(app.solution.is_none());
        assert!(!app.show_original);
        assert_eq!(app.displayed_values(), Some(&original));

        app.path = "missing-puzzle.json".into();
        app.load();
        assert!(app.is_error);
        assert_eq!(app.displayed_values(), Some(&original));
        assert_eq!(app.loaded_path, "puzzles/cat/medium/puzzle_aa.json");
    }

    #[test]
    fn worker_disconnect_is_recoverable() {
        let (sender, receiver) = mpsc::channel();
        let mut app = SinglenumApp {
            worker: Some(receiver),
            ..Default::default()
        };
        drop(sender);
        app.poll_worker();
        assert!(app.worker.is_none());
        assert!(app.is_error);
    }

    #[test]
    fn solves_on_worker_and_reports_errors() {
        let mut values: Vec<usize> = (0..81)
            .map(|i| (i / 9 * 3 + i / 27 + i % 9) % 9 + 1)
            .collect();
        values[0] = 0;
        let mut app = SinglenumApp {
            puzzle: Some(Puzzle::new(values).unwrap()),
            ..Default::default()
        };
        app.start_solve(&egui::Context::default());
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while app.worker.is_some() && std::time::Instant::now() < deadline {
            app.poll_worker();
            std::thread::sleep(Duration::from_millis(1));
        }
        assert!(app.worker.is_none(), "Worker did not finish in time");
        assert!(!app.is_error, "{}", app.status);
        assert_eq!(app.solution.unwrap()[0], 1);
        assert_eq!(app.puzzle.as_ref().unwrap().values()[0], 0);

        app.reset();
        let (sender, receiver) = mpsc::channel();
        app.worker = Some(receiver);
        sender.send(Err(anyhow::anyhow!("test error"))).unwrap();
        app.poll_worker();
        assert!(app.worker.is_none());
        assert!(app.is_error);
        assert!(app.status.contains("test error"));
        assert!(app.solution.is_none());
    }

    #[test]
    fn worker_result_and_limit_do_not_expose_partial_boards() {
        let puzzle = Puzzle::new(vec![0; 81]).unwrap();
        let (sender, receiver) = mpsc::channel();
        let mut app = SinglenumApp {
            puzzle: Some(puzzle.clone()),
            worker: Some(receiver),
            ..Default::default()
        };
        sender.send(solve(&puzzle, 1)).unwrap();
        app.poll_worker();
        assert!(app.worker.is_none());
        assert!(app.solution.is_none());
        assert!(app.status.contains("Attempt limit"));
        assert_eq!(app.displayed_values(), Some(puzzle.values()));
    }
}
