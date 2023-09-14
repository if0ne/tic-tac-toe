use crate::models::{game_state::GameState, mark::Mark};

use super::player::Player;

pub struct ConsolePlayer {
    mark: Mark,
}

impl ConsolePlayer {
    pub fn new(mark: Mark) -> Self {
        Self { mark }
    }
}

impl ConsolePlayer {
    fn get_cell(&self, size: usize) -> Result<usize, &'static str> {
        let mut input_string = String::new();
        println!("Your move {}:", self.mark);
        std::io::stdin().read_line(&mut input_string).unwrap();

        let number = input_string
            .trim()
            .parse::<usize>()
            .map_err(|_| "Invalid input!")?;

        let (left, right) = (number / 10, number % 10);

        if (1..=size).contains(&left) && (1..=size).contains(&right) {
            Ok((left - 1) * size + right - 1)
        } else {
            Err("Invalid input!")
        }
    }
}

impl Player for ConsolePlayer {
    fn make_move(&self, game_state: &mut GameState) -> Result<(), &'static str> {
        let mut cell = self.get_cell(game_state.grid().size());

        while let Err(err) = cell {
            println!("{}", err);
            cell = self.get_cell(game_state.grid().size());
        }

        let cell = cell.unwrap();
        game_state.set_cell(cell, self.mark)?;

        Ok(())
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
