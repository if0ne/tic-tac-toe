use crate::models::game_state::GameState;

pub trait Renderer {
    fn render(&self, game_state: &GameState);
}
