use super::{cell::Cell, grid::Grid, mark::Mark};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameStatus {
    Continuous,
    Tie,
    Win(Mark),
}

#[derive(Clone, Debug)]
pub struct GameState {
    grid: Grid,
    current_mark: Mark,
}

impl GameState {
    pub fn new(grid: Grid) -> Self {
        let current_mark = Mark::X;

        Self { grid, current_mark }
    }

    pub fn next_turn(&mut self) {
        self.current_mark = self.current_mark.oppiste();
    }

    pub fn current_mark(&self) -> Mark {
        self.current_mark
    }

    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    pub fn set_cell(&mut self, cell: usize, mark: Mark) -> Result<(), &'static str> {
        self.grid.set_cell(cell, mark)
    }

    pub fn set_empty(&mut self, cell: usize) {
        self.grid.set_empty(cell)
    }

    pub fn is_finished(&self) -> GameStatus {
        if self.is_winning(Mark::X) {
            return GameStatus::Win(Mark::X);
        } else if self.is_winning(Mark::O) {
            return GameStatus::Win(Mark::O);
        }

        let spots = self.grid.get_empty_indices();

        if spots.is_empty() {
            GameStatus::Tie
        } else {
            GameStatus::Continuous
        }
    }

    pub fn is_winning(&self, player: Mark) -> bool {
        // check by row and column
        for i in 0..self.grid.size() {
            if self.check_row(i, player) {
                return true;
            }

            if self.check_column(i, player) {
                return true;
            }
        }

        // primary diagonal
        if self.check_primary_diagonal(player) {
            return true;
        }

        // secondary diagonal
        if self.check_secondary_diagonal(player) {
            return true;
        }

        false
    }

    fn check_sequence(&self, offset: usize, step: usize, player: Mark) -> bool {
        self.grid
            .grid()
            .iter()
            .skip(offset)
            .step_by(step)
            .take(self.grid.size())
            .all(|cell| {
                if let Cell::Mark(cell) = cell {
                    player.eq(cell)
                } else {
                    false
                }
            })
    }

    fn check_row(&self, row: usize, player: Mark) -> bool {
        self.check_sequence(row * self.grid.size(), 1, player)
    }

    fn check_column(&self, column: usize, player: Mark) -> bool {
        self.check_sequence(column, self.grid.size(), player)
    }

    fn check_primary_diagonal(&self, player: Mark) -> bool {
        self.check_sequence(0, self.grid.size() + 1, player)
    }

    fn check_secondary_diagonal(&self, player: Mark) -> bool {
        self.check_sequence(self.grid.size() - 1, self.grid.size() - 1, player)
    }
}

#[allow(unused_must_use)]
#[cfg(test)]
mod tests {
    use crate::models::grid::Grid;

    use super::{GameState, Mark};

    #[test]
    fn check_row_1_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(0, mark);
        grid.set_cell(1, mark);
        grid.set_cell(2, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_row(0, mark));
    }

    #[test]
    fn check_row_2_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(3, mark);
        grid.set_cell(4, mark);
        grid.set_cell(5, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_row(1, mark));
    }

    #[test]
    fn check_row_3_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(6, mark);
        grid.set_cell(7, mark);
        grid.set_cell(8, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_row(2, mark));
    }

    #[test]
    fn check_column_1_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(0, mark);
        grid.set_cell(3, mark);
        grid.set_cell(6, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_column(0, mark));
    }

    #[test]
    fn check_column_2_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(1, mark);
        grid.set_cell(4, mark);
        grid.set_cell(7, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_column(1, mark));
    }

    #[test]
    fn check_column_3_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(2, mark);
        grid.set_cell(5, mark);
        grid.set_cell(8, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_column(2, mark));
    }

    #[test]
    fn check_primary_diagonal_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(0, mark);
        grid.set_cell(4, mark);
        grid.set_cell(8, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_primary_diagonal(mark));
    }

    #[test]
    fn check_secondary_diagonal_test() {
        let mark = Mark::X;
        let mut grid = Grid::new(3);

        grid.set_cell(2, mark);
        grid.set_cell(4, mark);
        grid.set_cell(6, mark);

        let game_state = GameState::new(grid);

        assert!(game_state.check_secondary_diagonal(mark));
    }
}
