use rand::Rng;
use tic_tac_toe::{
    game::{PlayerType, TicTacToe},
    models::mark::Mark,
};

fn main() {
    let map_size = get_map_size();
    let mode = get_mode();

    let mut game = TicTacToe::new_console_mod(mode.0, mode.1, map_size);

    game.run();
}

pub fn get_map_size() -> usize {
    let mut input_string = String::new();
    let variant = loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
        input_string.clear();

        let mut input_string = String::new();
        println!("Choose map size:");
        println!("1) 3x3");
        println!("2) 4x4");
        println!("3) 5x5");
        std::io::stdin().read_line(&mut input_string).unwrap();

        let map = input_string.trim().parse::<usize>();

        match map {
            Ok(value) => {
                if (1..=3).contains(&value) {
                    break value;
                } else {
                    println!("Please input number from 1 to 3");
                }
            }
            Err(err) => println!("{}", err),
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    };

    match variant {
        1 => 3,
        2 => 4,
        3 => 5,
        _ => unreachable!(),
    }
}

pub fn get_mode() -> (PlayerType, PlayerType) {
    let mut input_string = String::new();
    let variant = loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear screen
        input_string.clear();

        println!("Choose game mode:");
        println!("1) Player Versus Player");
        println!("2) Player Versus Computer");
        println!("3) Computer Versus Computer");
        std::io::stdin().read_line(&mut input_string).unwrap();

        let map = input_string.trim().parse::<usize>();

        match map {
            Ok(value) => {
                if (1..=3).contains(&value) {
                    break value;
                } else {
                    println!("Please input number from 1 to 3");
                }
            }
            Err(err) => println!("{}", err),
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    };

    match variant {
        1 => (PlayerType::Human(Mark::X), PlayerType::Human(Mark::O)),
        2 => {
            if rand::thread_rng().gen_bool(0.5) {
                (PlayerType::Human(Mark::X), PlayerType::Ai(Mark::O))
            } else {
                (PlayerType::Ai(Mark::X), PlayerType::Human(Mark::O))
            }
        }
        3 => (PlayerType::Ai(Mark::X), PlayerType::Ai(Mark::O)),
        _ => unreachable!(),
    }
}
