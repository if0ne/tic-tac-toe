use crate::models::cell::Cell;

use super::mark::Mark;

#[derive(Clone, Debug)]
pub struct Grid {
    map: Vec<Cell>,
    size: usize,
}

impl Grid {
    pub fn new(map_size: usize) -> Self {
        assert!(map_size >= 3);
        assert!(map_size <= 5);

        Self {
            map: vec![Cell::Empty; map_size * map_size],
            size: map_size,
        }
    }

    pub fn grid(&self) -> &[Cell] {
        &self.map
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_cell(&mut self, cell: usize, mark: Mark) -> Result<(), &'static str> {
        if self.map[cell] == Cell::Empty {
            self.map[cell] = Cell::Mark(mark);
            Ok(())
        } else {
            Err("It's not an empty cell")
        }
    }

    pub fn set_empty(&mut self, cell: usize) {
        self.map[cell] = Cell::Empty
    }

    pub fn get_empty_indices(&self) -> Vec<usize> {
        self.map
            .iter()
            .enumerate()
            .filter(|(_, cell)| **cell == Cell::Empty)
            .map(|(index, _)| index)
            .collect()
    }
}
