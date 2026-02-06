/// ColorMap editor widget for interactive gradient editing
/// 
/// Provides gradient preview, color stop editing, and interactive controls

use eframe::egui;
use scala_chromatica::{ColorMap, ColorStop};
use std::sync::Arc;

/// State for the colormap editor
#[derive(Default)]
pub struct ColorEditor {
    pub selected_stop_index: Option<usize>,
    pub temp_rgb: [u8; 3],
    pub temp_position: f64,
}

impl ColorEditor {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Render gradient preview bar showing the colormap
pub fn render_gradient_preview(
    ui: &mut egui::Ui,
    colormap: &ColorMap,
    width: f32,
    height: f32,
) {
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());

    // Sample the gradient at many points for smooth display
    let num_samples = width as usize;
    let mut mesh = egui::epaint::Mesh::default();

    for i in 0..num_samples {
        let t = i as f64 / (num_samples - 1) as f64;
        let color = colormap.get_color(t);

        let x = rect.min.x + (t as f32) * width;
        let color32 = egui::Color32::from_rgb(color.r, color.g, color.b);

        // Create a vertical strip
        let strip_width = width / num_samples as f32 + 1.0;
        let strip_rect = egui::Rect::from_min_size(
            egui::pos2(x, rect.min.y),
            egui::vec2(strip_width, height)
        );

        mesh.add_colored_rect(strip_rect, color32);
    }

    ui.painter().add(egui::Shape::Mesh(Arc::new(mesh)));

    // Draw border using line segments
    ui.painter().line_segment(
        [rect.min, egui::pos2(rect.max.x, rect.min.y)],
        egui::Stroke::new(1.0, egui::Color32::GRAY)
    );
    ui.painter().line_segment(
        [egui::pos2(rect.max.x, rect.min.y), rect.max],
        egui::Stroke::new(1.0, egui::Color32::GRAY)
    );
    ui.painter().line_segment(
        [rect.max, egui::pos2(rect.min.x, rect.max.y)],
        egui::Stroke::new(1.0, egui::Color32::GRAY)
    );
    ui.painter().line_segment(
        [egui::pos2(rect.min.x, rect.max.y), rect.min],
        egui::Stroke::new(1.0, egui::Color32::GRAY)
    );
}

/// Render colormap information
pub fn render_colormap_info(
    ui: &mut egui::Ui,
    colormap: &ColorMap,
) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Name:").strong());
        ui.label(&colormap.name);
    });
    
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Stops:").strong());
        ui.label(format!("{}", colormap.stops.len()));
    });
}

/// Render color stops list with edit controls
pub fn render_color_stops_list(
    ui: &mut egui::Ui,
    colormap: &mut ColorMap,
    editor: &mut ColorEditor,
) -> bool {
    let mut changed = false;
    let mut stop_to_remove: Option<usize> = None;

    ui.label(egui::RichText::new("Color Stops").strong());
    ui.add_space(5.0);

    let stops = colormap.stops.to_vec();

    for (i, stop) in stops.iter().enumerate() {
        ui.horizontal(|ui| {
            // Position
            ui.label(format!("{:.2}", stop.position));

            // Color preview box
            let color_size = egui::vec2(30.0, 20.0);
            let (color_rect, _) = ui.allocate_exact_size(color_size, egui::Sense::hover());
            ui.painter().rect_filled(
                color_rect,
                2.0,
                egui::Color32::from_rgb(stop.color.r, stop.color.g, stop.color.b),
            );
            // Border
            ui.painter().line_segment(
                [color_rect.min, egui::pos2(color_rect.max.x, color_rect.min.y)],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [egui::pos2(color_rect.max.x, color_rect.min.y), color_rect.max],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [color_rect.max, egui::pos2(color_rect.min.x, color_rect.max.y)],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [egui::pos2(color_rect.min.x, color_rect.max.y), color_rect.min],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );

            // RGB values
            ui.label(format!(
                "RGB({},{},{})",
                stop.color.r, stop.color.g, stop.color.b
            ));

            // Edit button
            if ui.small_button("✏").on_hover_text("Edit").clicked() {
                editor.selected_stop_index = Some(i);
                editor.temp_rgb = [stop.color.r, stop.color.g, stop.color.b];
                editor.temp_position = stop.position;
            }

            // Delete button (keep at least 2 stops)
            if stops.len() > 2 {
                if ui.small_button("🗑").on_hover_text("Delete").clicked() {
                    stop_to_remove = Some(i);
                    changed = true;
                }
            }
        });
    }

    // Remove stop if requested
    if let Some(index) = stop_to_remove {
        colormap.remove_stop(index);
        if let Some(selected) = editor.selected_stop_index {
            if selected == index {
                editor.selected_stop_index = None;
            } else if selected > index {
                editor.selected_stop_index = Some(selected - 1);
            }
        }
    }

    changed
}

/// Render color picker for editing selected stop
pub fn render_color_picker(
    ui: &mut egui::Ui,
    colormap: &mut ColorMap,
    editor: &mut ColorEditor,
) -> bool {
    let mut changed = false;

    if let Some(index) = editor.selected_stop_index {
        ui.group(|ui| {
            ui.label(egui::RichText::new("Edit Color Stop").strong());
            ui.add_space(5.0);

            // Position slider
            ui.label("Position:");
            if ui
                .add(egui::Slider::new(&mut editor.temp_position, 0.0..=1.0).fixed_decimals(2))
                .changed()
            {
                // Apply change
                if let Some(stop) = colormap.stops.get_mut(index) {
                    stop.position = editor.temp_position;
                    changed = true;
                }
            }

            ui.add_space(5.0);
            ui.label("Color (RGB):");

            // RGB sliders
            ui.horizontal(|ui| {
                ui.label("R:");
                if ui
                    .add(egui::Slider::new(&mut editor.temp_rgb[0], 0..=255).fixed_decimals(0))
                    .changed()
                {
                    if let Some(stop) = colormap.stops.get_mut(index) {
                        stop.color.r = editor.temp_rgb[0];
                        changed = true;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("G:");
                if ui
                    .add(egui::Slider::new(&mut editor.temp_rgb[1], 0..=255).fixed_decimals(0))
                    .changed()
                {
                    if let Some(stop) = colormap.stops.get_mut(index) {
                        stop.color.g = editor.temp_rgb[1];
                        changed = true;
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("B:");
                if ui
                    .add(egui::Slider::new(&mut editor.temp_rgb[2], 0..=255).fixed_decimals(0))
                    .changed()
                {
                    if let Some(stop) = colormap.stops.get_mut(index) {
                        stop.color.b = editor.temp_rgb[2];
                        changed = true;
                    }
                }
            });

            // Color preview
            ui.add_space(5.0);
            ui.label("Preview:");
            let preview_size = egui::vec2(ui.available_width(), 40.0);
            let (preview_rect, _) = ui.allocate_exact_size(preview_size, egui::Sense::hover());
            ui.painter().rect_filled(
                preview_rect,
                4.0,
                egui::Color32::from_rgb(
                    editor.temp_rgb[0],
                    editor.temp_rgb[1],
                    editor.temp_rgb[2]
                ),
            );
            // Border
            ui.painter().line_segment(
                [preview_rect.min, egui::pos2(preview_rect.max.x, preview_rect.min.y)],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [egui::pos2(preview_rect.max.x, preview_rect.min.y), preview_rect.max],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [preview_rect.max, egui::pos2(preview_rect.min.x, preview_rect.max.y)],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );
            ui.painter().line_segment(
                [egui::pos2(preview_rect.min.x, preview_rect.max.y), preview_rect.min],
                egui::Stroke::new(1.0, egui::Color32::GRAY)
            );

            ui.add_space(5.0);

            // Done/Cancel buttons
            ui.horizontal(|ui| {
                if ui.button("✓ Done").clicked() {
                    editor.selected_stop_index = None;
                }
                if ui.button("✖ Cancel").clicked() {
                    editor.selected_stop_index = None;
                }
            });
        });
    }

    changed
}

/// Render add stop button
pub fn render_add_stop_button(
    ui: &mut egui::Ui,
    colormap: &mut ColorMap,
) -> bool {
    if ui.button("➕ Add Color Stop").clicked() {
        // Add stop at midpoint with interpolated color
        let new_position = 0.5;
        let new_color = colormap.get_color(new_position);
        colormap.add_stop(ColorStop {
            position: new_position,
            color: new_color,
            name: None,
        });
        return true;
    }
    false
}
