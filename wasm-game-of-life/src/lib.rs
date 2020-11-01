mod utils;

use std::fmt::{self, Display, Formatter};

use rand::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static TITLE: &str = "Web Assembly Game of Life";

#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cell::Dead => write!(f, "◻"),
            Cell::Alive => write!(f, "◼"),
        }
    }
}

#[wasm_bindgen]
pub fn something() -> Cell {
    Cell::Dead
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(size: u32) -> Self {
        let mut cells = vec![Cell::Dead; (size * size) as usize];

        Self {
            width: size,
            height: size,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, column);
                if matches!(cell, Cell::Alive) {
                    if live_neighbors < 2 || live_neighbors > 3 {
                        next[index] = Cell::Dead;
                    }
                } else {
                    if live_neighbors == 3 {
                        next[index] = Cell::Alive;
                    }
                }
            }
        }

        self.cells = next;
    }

    pub fn randomize(&mut self) {
        self.privately_randomize();
    }

    fn privately_randomize(&mut self) {
        let mut rng = rand::thread_rng();

        self.cells = self
            .cells
            .iter()
            .map(|cell| {
                if rng.gen_range(0, 100) > 65 {
                    Cell::Alive
                } else {
                    *cell
                }
            })
            .collect();
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let neighbors = vec![
            self.get_index_above(row, column),
            self.get_index_above_right(row, column),
            self.get_index_right(row, column),
            self.get_index_below_right(row, column),
            self.get_index_below(row, column),
            self.get_index_below_left(row, column),
            self.get_index_left(row, column),
            self.get_index_above_left(row, column),
        ];

        neighbors.iter().fold(0, |mut count, next| {
            count
                + if let Some(index) = next {
                    self.cells[*index] as usize
                } else {
                    0
                }
        }) as u8
    }

    fn get_index_above(&self, row: u32, column: u32) -> Option<usize> {
        if row == 0 {
            None
        } else {
            Some(self.get_index(row - 1, column))
        }
    }

    fn get_index_right(&self, row: u32, column: u32) -> Option<usize> {
        if column >= self.width - 1 {
            None
        } else {
            Some(self.get_index(row, column + 1))
        }
    }

    fn get_index_below(&self, row: u32, column: u32) -> Option<usize> {
        if row >= self.height - 1 {
            None
        } else {
            Some(self.get_index(row + 1, column))
        }
    }

    fn get_index_left(&self, row: u32, column: u32) -> Option<usize> {
        if column == 0 {
            None
        } else {
            Some(self.get_index(row, column - 1))
        }
    }

    fn get_index_above_right(&self, row: u32, column: u32) -> Option<usize> {
        if row == 0 || column >= self.width - 1 {
            None
        } else {
            Some(self.get_index(row - 1, column + 1))
        }
    }

    fn get_index_below_right(&self, row: u32, column: u32) -> Option<usize> {
        if row == self.height - 1 || column == self.width - 1 {
            None
        } else {
            Some(self.get_index(row + 1, column + 1))
        }
    }

    fn get_index_below_left(&self, row: u32, column: u32) -> Option<usize> {
        if row == self.height - 1 || column == 0 {
            None
        } else {
            Some(self.get_index(row + 1, column - 1))
        }
    }

    fn get_index_above_left(&self, row: u32, column: u32) -> Option<usize> {
        if row == 0 || column == 0 {
            None
        } else {
            Some(self.get_index(row - 1, column - 1))
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for cell in line {
                write!(f, "{}", cell);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_index() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        let row = 3;
        let column = 3;
        let expected_result: usize = 18;

        assert_eq!(expected_result, universe.get_index(row, column));
    }

    #[test]
    fn test_get_index_above() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(13, universe.get_index_above(3, 3).unwrap());
        assert_eq!(None, universe.get_index_above(0, 3));
    }

    #[test]
    fn test_get_index_right() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(19, universe.get_index_right(3, 3).unwrap());
        assert_eq!(None, universe.get_index_right(3, 4));
    }

    #[test]
    fn test_get_index_below() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(23, universe.get_index_below(3, 3).unwrap());
        assert_eq!(None, universe.get_index_below(4, 3));
    }

    #[test]
    fn test_get_index_left() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(17, universe.get_index_left(3, 3).unwrap());
        assert_eq!(None, universe.get_index_left(3, 0));
    }

    #[test]
    fn test_get_index_above_right() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(14, universe.get_index_above_right(3, 3).unwrap());
        assert_eq!(None, universe.get_index_above_right(1, 4));
        assert_eq!(None, universe.get_index_above_right(0, 2));
    }

    #[test]
    fn test_get_index_below_right() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(24, universe.get_index_below_right(3, 3).unwrap());
        assert_eq!(None, universe.get_index_below_right(3, 4));
        assert_eq!(None, universe.get_index_below_right(4, 3));
    }

    #[test]
    fn test_get_index_below_left() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(22, universe.get_index_below_left(3, 3).unwrap());
        assert_eq!(None, universe.get_index_below_left(3, 0));
        assert_eq!(None, universe.get_index_below_left(4, 3));
    }

    #[test]
    fn test_get_index_above_left() {
        let universe = Universe {
            width: 5,
            height: 5,
            cells: vec![],
        };

        assert_eq!(12, universe.get_index_above_left(3, 3).unwrap());
        assert_eq!(None, universe.get_index_above_left(0, 3));
        assert_eq!(None, universe.get_index_above_left(3, 0));
    }

    #[test]
    fn test_live_neighbors_count() {
        // [
        //     [0, 1, 1],
        //     [1, 0, 1],
        //     [0, 0, 1],
        // ]
        let cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
        ];
        let universe = Universe {
            width: 3,
            height: 3,
            cells,
        };

        assert_eq!(5, universe.live_neighbor_count(1, 1));
    }

    #[test]
    fn test_tick() {
        // [
        //     [0, 1, 1],
        //     [1, 0, 1],
        //     [0, 0, 1],
        // ]
        let initial_cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
        ];
        // [
        //     [0, 1, 1],
        //     [0, 0, 1],
        //     [0, 1, 0],
        // ]
        let after_cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
        ];
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells: initial_cells,
        };
        universe.tick();
        assert_eq!(after_cells, universe.cells);
    }

    #[test]
    fn test_render_for_universe() {
        // [
        //     [0, 1, 1],
        //     [0, 0, 1],
        //     [0, 1, 0],
        // ]
        let cells = vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
        ];
        let mut universe = Universe {
            width: 3,
            height: 3,
            cells,
        };
        let expected_result = "◻◼◼\n◻◻◼\n◻◼◻\n";
        assert_eq!(expected_result, universe.render());
    }

    #[test]
    fn test_display_for_cell() {
        let alive_cell = Cell::Alive;
        let dead_cell = Cell::Dead;

        assert_eq!("◼", alive_cell.to_string());
        assert_eq!("◻", dead_cell.to_string());
    }

    #[test]
    fn test_new_universe() {
        let universe = Universe::new(3);

        assert_eq!(3, universe.width);
        assert_eq!(3, universe.height);
        assert_eq!(
            vec![
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
                Cell::Dead,
            ],
            universe.cells
        );
    }
}
