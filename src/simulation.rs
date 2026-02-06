/// Physics module - handles the sand falling simulation
/// 
/// This module implements the core simulation logic for falling sand particles.
/// Sand falls down following simple gravity rules.

use crate::grid::{Cell, Grid};

pub struct Simulation {
    grid: Grid,
    current_frame: u64,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            current_frame: 0,
        }
    }

    /// Get a reference to the grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get a mutable reference to the grid
    pub fn grid_mut(&mut self) -> &mut Grid {
        &mut self.grid
    }

    /// Get current frame number
    pub fn current_frame(&self) -> u64 {
        self.current_frame
    }

    /// Update the simulation by one step
    /// 
    /// This implements falling sand physics:
    /// - Sand falls straight down if space below is empty
    /// - Sand falls diagonally (left or right) if straight down is blocked
    /// - Sand stays in place if no moves are possible
    pub fn step(&mut self) {
        self.current_frame += 1;
        let width = self.grid.width;
        let height = self.grid.height;

        // Process from bottom to top to avoid processing same particle twice
        // Also process in alternating left-right/right-left order to reduce bias
        for y in (0..height - 1).rev() {
            // Alternate direction each row to reduce visual bias
            if y % 2 == 0 {
                // Left to right
                for x in 0..width {
                    self.update_cell(x, y);
                }
            } else {
                // Right to left
                for x in (0..width).rev() {
                    self.update_cell(x, y);
                }
            }
        }

        // Update stationary counters for all sand particles
        for y in 0..height {
            for x in 0..width {
                if self.grid.get(x, y) == Some(Cell::Sand) {
                    let metadata = &mut self.grid.metadata[[y, x]];
                    // Check if particle moved since last frame
                    if metadata.last_x == x && metadata.last_y == y {
                        metadata.stationary_frames += 1;
                    } else {
                        metadata.stationary_frames = 0;
                        metadata.last_x = x;
                        metadata.last_y = y;
                    }
                }
            }
        }
    }

    /// Update a single cell's position based on physics rules
    fn update_cell(&mut self, x: usize, y: usize) {
        if self.grid.get(x, y) != Some(Cell::Sand) {
            return;
        }

        let below_y = y + 1;

        // Try to move straight down
        if self.grid.get(x, below_y) == Some(Cell::Empty) {
            self.grid.set(x, y, Cell::Empty);
            self.grid.set(x, below_y, Cell::Sand);
            // Move metadata along with particle
            self.grid.metadata[[below_y, x]] = self.grid.metadata[[y, x]];
            return;
        }

        // Try to move diagonally down-left or down-right
        // Use a pseudo-random pattern based on position to avoid visual bias
        // Without this, sand would always prefer one direction creating artifacts
        let try_left_first = (x + y) % 2 == 0;

        if try_left_first {
            // Try moving diagonally down-left first
            if x > 0 && self.grid.get(x - 1, below_y) == Some(Cell::Empty) {
                // Space available down-left: move sand there
                self.grid.set(x, y, Cell::Empty);
                self.grid.set(x - 1, below_y, Cell::Sand);
                // Move metadata along with particle
                self.grid.metadata[[below_y, x - 1]] = self.grid.metadata[[y, x]];
                return;
            }
            // Down-left blocked, try moving diagonally down-right
            if x < self.grid.width - 1 && self.grid.get(x + 1, below_y) == Some(Cell::Empty) {
                // Space available down-right: move sand there
                self.grid.set(x, y, Cell::Empty);
                self.grid.set(x + 1, below_y, Cell::Sand);
                // Move metadata along with particle
                self.grid.metadata[[below_y, x + 1]] = self.grid.metadata[[y, x]];
                return;
            }
        } else {
            // Try moving diagonally down-right first
            if x < self.grid.width - 1 && self.grid.get(x + 1, below_y) == Some(Cell::Empty) {
                // Space available down-right: move sand there
                self.grid.set(x, y, Cell::Empty);
                self.grid.set(x + 1, below_y, Cell::Sand);
                // Move metadata along with particle
                self.grid.metadata[[below_y, x + 1]] = self.grid.metadata[[y, x]];
                return;
            }
            // Down-right blocked, try moving diagonally down-left
            if x > 0 && self.grid.get(x - 1, below_y) == Some(Cell::Empty) {
                // Space available down-left: move sand there
                self.grid.set(x, y, Cell::Empty);
                self.grid.set(x - 1, below_y, Cell::Sand);
                // Move metadata along with particle
                self.grid.metadata[[below_y, x - 1]] = self.grid.metadata[[y, x]];
                return;
            }
        }

        // Sand can't move, stays in place
    }
}
