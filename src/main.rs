mod grid;
mod simulation;

use eframe::egui;
use grid::Cell;
use simulation::Simulation;

const GRID_WIDTH: usize = 800;
const GRID_HEIGHT: usize = 600;
const CELL_SIZE: f32 = 1.0;
const BRUSH_RADIUS: i32 = 20;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                (GRID_WIDTH as f32 * CELL_SIZE) as f32,
                (GRID_HEIGHT as f32 * CELL_SIZE) as f32 + 40.0, // Extra space for controls
            ])
            .with_resizable(false),
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Arenae - Falling Sand Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(SandApp::new()))),
    )
}

struct SandApp {
    simulation: Simulation,
    paused: bool,
    steps_per_frame: usize,
}

impl SandApp {
    fn new() -> Self {
        Self {
            simulation: Simulation::new(GRID_WIDTH, GRID_HEIGHT),
            paused: false,
            steps_per_frame: 1,
        }
    }

    /// Handle mouse interactions for adding/removing sand
    fn handle_mouse_input(&mut self, ui: &egui::Ui, painter_pos: egui::Pos2) {
        let response = ui.interact(
            egui::Rect::from_min_size(
                painter_pos,
                egui::vec2(
                    GRID_WIDTH as f32 * CELL_SIZE,
                    GRID_HEIGHT as f32 * CELL_SIZE,
                ),
            ),
            ui.id().with("canvas"),
            egui::Sense::click_and_drag(),
        );

        if let Some(pos) = response.hover_pos() {
            let grid_x = ((pos.x - painter_pos.x) / CELL_SIZE) as i32;
            let grid_y = ((pos.y - painter_pos.y) / CELL_SIZE) as i32;

            // Left click or left drag: add sand
            if ui.input(|i| i.pointer.primary_down()) {
                self.simulation
                    .grid_mut()
                    .add_sand_circle(grid_x, grid_y, BRUSH_RADIUS);
            }

            // Right click or right drag: remove sand
            if ui.input(|i| i.pointer.secondary_down()) {
                self.simulation
                    .grid_mut()
                    .remove_sand_circle(grid_x, grid_y, BRUSH_RADIUS);
            }
        }
    }

    /// Render the grid to the screen
    fn render_grid(&self, painter: &egui::Painter, painter_pos: egui::Pos2) {
        let grid = self.simulation.grid();

        // Draw background
        painter.rect_filled(
            egui::Rect::from_min_size(
                painter_pos,
                egui::vec2(
                    GRID_WIDTH as f32 * CELL_SIZE,
                    GRID_HEIGHT as f32 * CELL_SIZE,
                ),
            ),
            0.0,
            egui::Color32::from_rgb(20, 20, 30),
        );

        // Draw sand cells
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if grid.get(x, y) == Some(Cell::Sand) {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(
                            painter_pos.x + x as f32 * CELL_SIZE,
                            painter_pos.y + y as f32 * CELL_SIZE,
                        ),
                        egui::vec2(CELL_SIZE, CELL_SIZE),
                    );
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(230, 200, 120));
                }
            }
        }
    }
}

impl eframe::App for SandApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints for smooth animation
        ctx.request_repaint();

        // Run simulation steps if not paused
        if !self.paused {
            for _ in 0..self.steps_per_frame {
                self.simulation.step();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Controls
            ui.horizontal(|ui| {
                ui.label("Controls:");
                if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                    self.paused = !self.paused;
                }
                if ui.button("🗑 Clear").clicked() {
                    self.simulation.grid_mut().clear();
                }
                ui.label("|");
                ui.label("Left Click: Add Sand");
                ui.label("|");
                ui.label("Right Click: Remove Sand");
            });

            ui.add_space(5.0);

            // Canvas for rendering
            let (response, painter) = ui.allocate_painter(
                egui::vec2(
                    GRID_WIDTH as f32 * CELL_SIZE,
                    GRID_HEIGHT as f32 * CELL_SIZE,
                ),
                egui::Sense::click_and_drag(),
            );

            let painter_pos = response.rect.min;

            // Render the grid
            self.render_grid(&painter, painter_pos);

            // Handle mouse input
            self.handle_mouse_input(ui, painter_pos);
        });
    }
}

