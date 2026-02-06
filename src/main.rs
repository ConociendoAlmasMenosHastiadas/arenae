mod grid;
mod simulation;
mod colors;
mod colormap_editor;

use eframe::egui;
use grid::Cell;
use simulation::Simulation;
use colors::{ColorMode, list_available_colormaps, load_colormap, calculate_color};
use scala_chromatica::ColorMap;
use colormap_editor::ColorEditor;

const GRID_WIDTH: usize = 800;
const GRID_HEIGHT: usize = 600;
const CELL_SIZE: f32 = 1.0;
const BRUSH_RADIUS: i32 = 20;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([
                1100.0, // 300px sidebar + 800px canvas
                600.0,
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
    // Color settings
    color_mode: ColorMode,
    available_colormaps: Vec<String>,
    current_colormap_name: String,
    current_colormap: ColorMap,
    frame_period: u64,
    // Background color
    background_color: [u8; 3],
    // Colormap editor
    color_editor: ColorEditor,
}

impl SandApp {
    fn new() -> Self {
        let available_colormaps = list_available_colormaps();
        let colormap_name = available_colormaps.first()
            .cloned()
            .unwrap_or_else(|| "Fire".to_string());
        let colormap = load_colormap(&colormap_name)
            .unwrap_or_else(|_| {
                // Fallback: load Fire
                scala_chromatica::io::load_builtin_colormap("Fire")
                    .expect("Failed to load fallback colormap")
            });
        
        Self {
            simulation: Simulation::new(GRID_WIDTH, GRID_HEIGHT),
            paused: false,
            steps_per_frame: 1,
            color_mode: ColorMode::CreationTime,
            available_colormaps,
            current_colormap_name: colormap_name,
            current_colormap: colormap,
            frame_period: 500,
            background_color: [20, 20, 30],
            color_editor: ColorEditor::new(),
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
                let frame = self.simulation.current_frame();
                self.simulation
                    .grid_mut()
                    .add_sand_circle(grid_x, grid_y, BRUSH_RADIUS, frame);
            }

            // Right click or right drag: remove sand
            if ui.input(|i| i.pointer.secondary_down()) {
                self.simulation
                    .grid_mut()
                    .remove_sand_circle(grid_x, grid_y, BRUSH_RADIUS);
            }
        }
    }

    fn render_grid(&self, painter: &egui::Painter, painter_pos: egui::Pos2) {
        let grid = self.simulation.grid();
        let current_frame = self.simulation.current_frame();

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
            egui::Color32::from_rgb(
                self.background_color[0],
                self.background_color[1],
                self.background_color[2]
            ),
        );

        // Draw sand cells with colors
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if grid.get(x, y) == Some(Cell::Sand) {
                    let metadata = &grid.metadata[[y, x]];
                    let color = calculate_color(
                        self.color_mode,
                        &self.current_colormap,
                        metadata,
                        current_frame,
                        self.frame_period,
                        x,
                        y,
                        GRID_HEIGHT,
                    );

                    let rect = egui::Rect::from_min_size(
                        egui::pos2(
                            painter_pos.x + x as f32 * CELL_SIZE,
                            painter_pos.y + y as f32 * CELL_SIZE,
                        ),
                        egui::vec2(CELL_SIZE, CELL_SIZE),
                    );
                    painter.rect_filled(rect, 0.0, color);
                }
            }
        }
    }
}

impl eframe::App for SandApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Only request continuous repaints when simulation is running
        if !self.paused {
            ctx.request_repaint();
        }

        // Run simulation steps if not paused
        if !self.paused {
            for _ in 0..self.steps_per_frame {
                self.simulation.step();
            }
        }

        // Left sidebar with color controls
        egui::SidePanel::left("controls")
            .default_width(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.heading("Arenae Controls");
                        ui.add_space(10.0);

                        // Simulation controls
                        ui.label(egui::RichText::new("Simulation").strong());
                        ui.add_space(5.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button(if self.paused { "▶ Resume" } else { "⏸ Pause" }).clicked() {
                                self.paused = !self.paused;
                            }
                            if ui.button("🗑 Clear").clicked() {
                                self.simulation.grid_mut().clear();
                            }
                        });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Color settings
                        ui.label(egui::RichText::new("Color Settings").strong());
                        ui.add_space(5.0);

                        // Colormap selector
                        ui.label("Colormap:");
                        egui::ComboBox::from_label("")
                            .selected_text(&self.current_colormap_name)
                            .show_ui(ui, |ui| {
                                for colormap_name in self.available_colormaps.iter() {
                                    if ui.selectable_label(
                                        self.current_colormap_name == *colormap_name,
                                        colormap_name
                                    ).clicked() {
                                        if let Ok(loaded_colormap) = load_colormap(colormap_name) {
                                            self.current_colormap = loaded_colormap;
                                            self.current_colormap_name = colormap_name.clone();
                                        }
                                    }
                                }
                            });

                        ui.add_space(10.0);

                        // Color mode selector
                        ui.label("Coloring Mode:");
                        egui::ComboBox::from_id_salt("color_mode")
                            .selected_text(match self.color_mode {
                                ColorMode::CreationTime => "Creation Time",
                                ColorMode::Age => "Age (Stationary Time)",
                                ColorMode::Height => "Height",
                                ColorMode::Random => "Random",
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.color_mode, ColorMode::CreationTime, "Creation Time");
                                ui.selectable_value(&mut self.color_mode, ColorMode::Age, "Age (Stationary Time)");
                                ui.selectable_value(&mut self.color_mode, ColorMode::Height, "Height");
                                ui.selectable_value(&mut self.color_mode, ColorMode::Random, "Random");
                            });

                        ui.add_space(10.0);

                        // Frame period slider
                        ui.label("Color Cycle Period (frames):");
                        ui.add(egui::Slider::new(&mut self.frame_period, 100..=2000)
                            .text("frames"));

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Background color picker
                        ui.label(egui::RichText::new("Background Color").strong());
                        ui.add_space(5.0);

                        ui.horizontal(|ui| {
                            ui.label("R:");
                            ui.add(egui::Slider::new(&mut self.background_color[0], 0..=255)
                                .fixed_decimals(0));
                        });

                        ui.horizontal(|ui| {
                            ui.label("G:");
                            ui.add(egui::Slider::new(&mut self.background_color[1], 0..=255)
                                .fixed_decimals(0));
                        });

                        ui.horizontal(|ui| {
                            ui.label("B:");
                            ui.add(egui::Slider::new(&mut self.background_color[2], 0..=255)
                                .fixed_decimals(0));
                        });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Colormap Editor
                        ui.collapsing("🎨 Edit Colormap", |ui| {
                            ui.add_space(5.0);

                            // Colormap info
                            colormap_editor::render_colormap_info(ui, &self.current_colormap);
                            ui.add_space(10.0);

                            // Gradient preview
                            colormap_editor::render_gradient_preview(
                                ui,
                                &self.current_colormap,
                                ui.available_width(),
                                40.0
                            );
                            ui.add_space(10.0);

                            // Color stops list
                            colormap_editor::render_color_stops_list(
                                ui,
                                &mut self.current_colormap,
                                &mut self.color_editor
                            );
                            ui.add_space(10.0);

                            // Add stop button
                            colormap_editor::render_add_stop_button(
                                ui,
                                &mut self.current_colormap
                            );
                            ui.add_space(10.0);

                            // Color picker (if a stop is selected)
                            if self.color_editor.selected_stop_index.is_some() {
                                colormap_editor::render_color_picker(
                                    ui,
                                    &mut self.current_colormap,
                                    &mut self.color_editor
                                );
                            }
                        });

                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);

                        // Instructions
                        ui.label(egui::RichText::new("Mouse Controls").strong());
                        ui.add_space(5.0);
                        ui.label("Left Click/Drag: Add Sand");
                        ui.label("Right Click/Drag: Remove Sand");

                        ui.add_space(10.0);
                    });
            });

        // Main canvas
        egui::CentralPanel::default().show(ctx, |ui| {
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

