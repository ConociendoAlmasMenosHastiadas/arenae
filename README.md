# Arenae - Falling Sand Simulation

A falling sand simulation built in Rust as a learning project for GPU programming. The project follows a modular approach that will enable transitioning from CPU to GPU computation.

## Features

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

## Technical Details

- **Grid Size**: 200 x 150 cells
- **Cell Size**: 4 pixels
- **Brush Radius**: 5 cells
- **Physics**: Simple gravity with diagonal falling when blocked
- **Rendering**: egui with immediate mode GUI

## Roadmap

### v0.2.0 - Pretty Colors (Planned)
- Integrate scala-chromatica for color manipulation
- Add various color schemes and palettes
- Particle color variations based on age, height, or velocity
- UI controls for color scheme selection

### v0.3.0 - CPU Parallelization (Planned)
- Explore Rayon-based parallelization strategies
- Balance performance improvements with physics correctness
- Benchmark and optimize sequential code

### v0.4.0 - GPU Acceleration (Planned)
- Transition to GPU compute using wgpu
- Implement physics in compute shaders
- Scale to much larger grid sizes
- Direct GPU rendering for massive performance improvements

## Dependencies

- `ndarray` - Multi-dimensional array handling
- `eframe` - GUI framework wrapper
- `egui` - Immediate mode GUI

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
