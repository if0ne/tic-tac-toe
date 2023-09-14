use crate::models::{game_state::GameState, mark::Mark};

pub trait Player {
    fn make_move(&self, game_state: &mut GameState) -> Result<(), &'static str>;
    fn get_mark(&self) -> Mark;
}
