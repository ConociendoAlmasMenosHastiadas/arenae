/// Color schemes module - handles particle coloring
/// 
/// This module provides different coloring strategies for sand particles
/// using scala-chromatica's built-in colormaps

use scala_chromatica::ColorMap;

/// Coloring mode determines how particles are colored
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorMode {
    /// Color based on when particle was created (frame number)
    CreationTime,
    /// Color based on how long particle has been stationary
    Age,
    /// Color based on height (Y position)
    Height,
    /// Random color variation for texture
    Random,
}

/// Get list of all available colormaps (built-in + custom)
pub fn list_available_colormaps() -> Vec<String> {
    match scala_chromatica::io::list_available_colormaps() {
        Ok(infos) => infos.into_iter().map(|info| info.name).collect(),
        Err(_) => vec!["Default".to_string()], // Fallback
    }
}

/// Load a colormap by name
pub fn load_colormap(name: &str) -> Result<ColorMap, String> {
    scala_chromatica::io::load_colormap(name).map_err(|e| format!("Failed to load colormap: {}", e))
}

/// Particle metadata for coloring
#[derive(Clone, Copy, Debug)]
pub struct ParticleMetadata {
    /// Frame number when this particle was created
    pub creation_frame: u64,
    /// How many frames this particle has been stationary
    pub stationary_frames: u32,
    /// Last position (for detecting movement)
    pub last_x: usize,
    pub last_y: usize,
}

impl Default for ParticleMetadata {
    fn default() -> Self {
        Self {
            creation_frame: 0,
            stationary_frames: 0,
            last_x: 0,
            last_y: 0,
        }
    }
}

impl ParticleMetadata {
    pub fn new(frame: u64, x: usize, y: usize) -> Self {
        Self {
            creation_frame: frame,
            stationary_frames: 0,
            last_x: x,
            last_y: y,
        }
    }
}

/// Calculate color for a particle based on mode and metadata
pub fn calculate_color(
    mode: ColorMode,
    colormap: &ColorMap,
    metadata: &ParticleMetadata,
    _current_frame: u64,
    frame_period: u64,
    x: usize,
    y: usize,
    grid_height: usize,
) -> egui::Color32 {
    let t = match mode {
        ColorMode::CreationTime => {
            // Color based on when particle was created (fixed color per particle)
            // Frame period cycles the colormap as new particles are created
            (metadata.creation_frame % frame_period) as f64 / frame_period as f64
        }
        ColorMode::Age => {
            // Color based on how long stationary (0-100 frames mapped to 0-1)
            (metadata.stationary_frames as f64 / 100.0).min(1.0)
        }
        ColorMode::Height => {
            // Color based on Y position (bottom = 0, top = 1)
            1.0 - (y as f64 / grid_height as f64)
        }
        ColorMode::Random => {
            // Use creation frame + position as seed for consistent random
            let seed = (metadata.creation_frame + x as u64 + y as u64) % 1000;
            seed as f64 / 1000.0
        }
    };

    let color = colormap.get_color(t);
    egui::Color32::from_rgb(color.r, color.g, color.b)
}
