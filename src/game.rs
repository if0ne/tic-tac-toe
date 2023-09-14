use crate::{
    controller::{ai_player::AiPlayer, console_player::ConsolePlayer, player::Player},
    models::{
        game_state::{GameState, GameStatus},
        grid::Grid,
        mark::Mark,
    },
    view::{console_renderer::ConsoleRenderer, renderer::Renderer},
};

pub enum PlayerType {
    Human(Mark),
    Ai(Mark),
}

impl PlayerType {
    pub fn get_mark(&self) -> Mark {
        match self {
            PlayerType::Human(mark) => *mark,
            PlayerType::Ai(mark) => *mark,
        }
    }
}

pub struct TicTacToe {
    player1: Box<dyn Player>,
    player2: Box<dyn Player>,
    game_state: GameState,
    renderer: Box<dyn Renderer>,
}

impl TicTacToe {
    pub fn new_console_mod(player1: PlayerType, player2: PlayerType, map_size: usize) -> Self {
        assert_ne!(player1.get_mark(), player2.get_mark());

        let player1 = Self::get_player_class(player1);
        let player2 = Self::get_player_class(player2);

        let renderer = Box::new(ConsoleRenderer);

        Self {
            player1,
            player2,
            game_state: GameState::new(Grid::new(map_size)),
            renderer,
        }
    }

    pub fn get_player_class(player: PlayerType) -> Box<dyn Player> {
        match player {
            PlayerType::Human(mark) => Box::new(ConsolePlayer::new(mark)),
            PlayerType::Ai(mark) => Box::new(AiPlayer::new(mark)),
        }
    }

    pub fn run(&mut self) {
        while self.game_state.is_finished() == GameStatus::Continuous {
            self.renderer.render(&self.game_state);

            let res = if self.game_state.current_mark() == self.player1.get_mark() {
                self.player1.make_move(&mut self.game_state)
            } else {
                self.player2.make_move(&mut self.game_state)
            };

            if let Err(err) = res {
                println!("{}", err);
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }

            self.game_state.next_turn();
        }

        self.renderer.render(&self.game_state);

        match self.game_state.is_finished() {
            GameStatus::Continuous => todo!(),
            GameStatus::Tie => println!("Tie!"),
            GameStatus::Win(mark) => println!("Player {} WON!", mark),
        }
    }
}
