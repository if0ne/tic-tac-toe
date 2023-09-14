use rand::Rng;
use rayon::prelude::*;

use crate::models::{game_state::GameState, mark::Mark};

use super::player::Player;

pub struct AiPlayer {
    mark: Mark,
}

impl AiPlayer {
    const DEPTH_LIMIT: i32 = 10;

    pub fn new(mark: Mark) -> Self {
        Self { mark }
    }

    fn best_move(&self, game_state: &mut GameState) -> Option<usize> {
        let spots = game_state.grid().get_empty_indices();

        if spots.len() == game_state.grid().grid().len() {
            return Some(rand::thread_rng().gen_range(0..spots.len()));
        }

        spots.into_par_iter().max_by_key(|spot| {
            let mut game_state = game_state.clone();
            let current = game_state.current_mark();
            let depth = game_state.grid().size() as i32;
            let _ = game_state.set_cell(*spot, current);
            let score = Self::minimax(
                &mut game_state,
                current,
                current.oppiste(),
                Self::DEPTH_LIMIT - depth,
                i32::MIN,
                i32::MAX,
            );
            game_state.set_empty(*spot);

            score
        })
    }

    fn minimax(
        game_state: &mut GameState,
        to_win: Mark,
        next: Mark,
        depth: i32,
        alpha: i32,
        beta: i32,
    ) -> i32 {
        let spots = game_state.grid().get_empty_indices();

        if depth == 0 {
            return if to_win == next {
                Self::DEPTH_LIMIT
            } else {
                -Self::DEPTH_LIMIT
            };
        }

        if game_state.is_winning(to_win) {
            return 10 + depth;
        } else if game_state.is_winning(to_win.oppiste()) {
            return -depth - 10;
        } else if spots.is_empty() {
            return 0;
        }

        let mut best_score = if to_win == next { i32::MIN } else { i32::MAX };

        let mut alpha = alpha;
        let mut beta = beta;

        for spot in spots {
            let _ = game_state.set_cell(spot, next);
            let score = Self::minimax(game_state, to_win, next.oppiste(), depth - 1, alpha, beta);
            game_state.set_empty(spot);

            if to_win == next {
                best_score = best_score.max(score);
                alpha = alpha.max(score)
            } else {
                best_score = best_score.min(score);
                beta = beta.min(score);
            }

            if beta <= alpha {
                break;
            }
        }

        best_score
    }
}

impl Player for AiPlayer {
    fn make_move(
        &self,
        game_state: &mut crate::models::game_state::GameState,
    ) -> Result<(), &'static str> {
        let cell = self.best_move(game_state);

        if let Some(cell) = cell {
            let _ = game_state.set_cell(cell, self.mark);
            std::thread::sleep(std::time::Duration::from_secs(1));

            Ok(())
        } else {
            Err("No valid move!")
        }
    }

    fn get_mark(&self) -> Mark {
        self.mark
    }
}
