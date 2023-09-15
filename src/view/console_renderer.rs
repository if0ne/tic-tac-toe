use crate::{models::{cell::Cell, game_state::GameState, grid::Grid, mark::Mark}, util::clear_terminal_screen};

use super::renderer::Renderer;

impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mark::X => write!(f, "X"),
            Mark::O => write!(f, "O"),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::Mark(mark) => mark.fmt(f),
        }
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "    ")?;

        for i in 1..=self.size() {
            write!(f, "  {}  |", i)?;
        }

        writeln!(f)?;

        let splitter = "=".repeat(4 + 6 * self.size());

        writeln!(f, "{}", splitter)?;

        for i in 0..self.size() {
            write!(f, " {} |", i + 1)?;
            for j in 0..self.size() {
                if self.grid()[i * self.size() + j] == Cell::Empty {
                    write!(f, " {}{}  |", i + 1, j + 1)?;
                } else {
                    write!(f, "  {}  |", self.grid()[i * self.size() + j])?;
                }
            }
            writeln!(f)?;
            writeln!(f, "{}", splitter)?;
        }

        writeln!(f)
    }
}

pub struct ConsoleRenderer;

impl Renderer for ConsoleRenderer {
    fn render(&self, game_state: &GameState) {
        clear_terminal_screen();

        println!("{}", game_state.grid());
    }
}
