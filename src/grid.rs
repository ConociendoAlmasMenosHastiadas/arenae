/// Grid module - manages the sand simulation grid using ndarray
/// 
/// This module uses ndarray for 2D grid storage. We're evaluating if this is
/// the best choice for future GPU expansion (noted in v0.1.0 plan).

use ndarray::Array2;

/// Represents different cell types in the simulation
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Sand,
}

/// The simulation grid that holds the sand particles
#[derive(Clone)]
pub struct Grid {
    /// 2D array representing the grid state
    /// Shape is (height, width) - row-major order
    cells: Array2<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    /// Creates a new empty grid with the specified dimensions
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: Array2::from_elem((height, width), Cell::Empty),
            width,
            height,
        }
    }

    /// Gets the cell at the specified position
    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[[y, x]])
        } else {
            None
        }
    }

    /// Sets the cell at the specified position
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[[y, x]] = cell;
        }
    }

    /// Checks if a position is within bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    /// Gets a reference to the cells array for rendering
    pub fn cells(&self) -> &Array2<Cell> {
        &self.cells
    }

    /// Clears the entire grid
    pub fn clear(&mut self) {
        self.cells.fill(Cell::Empty);
    }

    /// Adds sand in a circular area centered at (cx, cy)
    pub fn add_sand_circle(&mut self, cx: i32, cy: i32, radius: i32) {
        let r_squared = radius * radius;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r_squared {
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.in_bounds(x, y) {
                        self.set(x as usize, y as usize, Cell::Sand);
                    }
                }
            }
        }
    }

    /// Removes sand in a circular area centered at (cx, cy)
    pub fn remove_sand_circle(&mut self, cx: i32, cy: i32, radius: i32) {
        let r_squared = radius * radius;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r_squared {
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.in_bounds(x, y) {
                        self.set(x as usize, y as usize, Cell::Empty);
                    }
                }
            }
        }
    }
}
