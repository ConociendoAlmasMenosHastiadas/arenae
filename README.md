# Arenae - Falling Sand Simulation

A falling sand simulation built in Rust as a learning project for GPU programming. The project follows a modular approach that will enable transitioning from CPU to GPU computation.

## Features

### v0.2.0 - Pretty Colors with Scala Chromatica
- ✅ Integrated scala-chromatica for professional color gradients
- ✅ 14+ built-in colormaps (Fire, Ocean, Electric Neon, Cosmic Dawn, etc.)
- ✅ Dynamic colormap discovery (built-in + custom)
- ✅ Multiple coloring modes:
  - Creation Time: Particles colored by when they spawned
  - Age: Color based on how long particle has been stationary
  - Height: Color gradient by Y position
  - Random: Consistent random colors for texture
- ✅ Adjustable color cycle period (100-2000 frames)
- ✅ Interactive colormap editor:
  - Live gradient preview
  - Edit/delete color stops
  - Add new color stops
  - RGB sliders with live preview
  - Colormap info display
- ✅ Customizable background color with RGB sliders
- ✅ Sidebar GUI with collapsible sections
- ✅ Grid upgraded to 800x600 @ 1px per cell

### v0.1.0 - CPU Implementation
- ✅ Interactive falling sand simulation running on CPU
- ✅ Grid-based physics using ndarray
- ✅ Real-time rendering with egui
- ✅ Mouse controls:
  - Left click/drag: Add sand particles
  - Right click/drag: Remove sand particles
- ✅ Simulation controls:
  - Pause/Resume button
  - Clear grid button

## Running the Project

```bash
cargo run --release
```

For development (unoptimized):
```bash
cargo run
```

## Project Structure

- `src/main.rs` - Application entry point and egui UI
- `src/grid.rs` - Grid data structure using ndarray
- `src/simulation.rs` - Physics simulation logic
- `src/colors.rs` - Color system and colormap integration
- `src/colormap_editor.rs` - Interactive colormap editing widgets

## Technical Details

- **Grid Size**: 800 x 600 cells (v0.2.0+) / 200 x 150 cells (v0.1.0)
- **Cell Size**: 1 pixel (v0.2.0+) / 4 pixels (v0.1.0)
- **Brush Radius**: 20 cells (v0.2.0+) / 5 cells (v0.1.0)
- **Physics**: Simple gravity with diagonal falling when blocked
- **Rendering**: egui with immediate mode GUI + scala-chromatica colormaps
- **Metadata Tracking**: Parallel Array2<ParticleMetadata> for color state

## Roadmap

### v0.3.0 - CPU Parallelization (Planned)
- Explore Rayon-based parallelization strategies
- Balance performance improvements with physics correctness
- Benchmark and optimize sequential code
- Study double-buffering vs sequential processing tradeoffs

### v0.4.0 - GPU Acceleration (Planned)
- Transition to GPU compute using wgpu
- Implement physics in compute shaders
- Scale to much larger grid sizes
- Direct GPU rendering for massive performance improvements

## Dependencies

- `ndarray` - Multi-dimensional array handling
- `eframe` - GUI framework wrapper
- `egui` - Immediate mode GUI
- `rayon` - Data parallelism (for future use)
- `scala-chromatica` - Color gradient library

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
