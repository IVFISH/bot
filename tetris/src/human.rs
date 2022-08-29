use crate::game::*;
use crate::placement::*;
use crate::players::*;
use crate::queue::PieceQueue;
use std::fmt::{Display, Formatter};

pub struct Human {
    pub game: Game,
    pub next_move: Option<String>,
}

impl Display for Human {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.game)?;
        Ok(())
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {
            game: Game::new(None),
            next_move: None,
        }
    }
}

impl Player for Human {
    fn get_game_mut(&mut self) -> &mut Game {
        &mut self.game
    }
    fn get_game(&self) -> &Game { &self.game}

    fn get_next_move(&mut self) -> MoveList {
        if let Some(eshan) = &self.next_move {
            let out = vec![string_to_command(eshan.clone())];
            self.next_move = None;
            return out;
        }
        return vec![Command::None];
    }
}

impl Human {
    pub fn set_next_move(&mut self, eshan: String) {
        self.next_move = Some(eshan);
    }
}

fn string_to_command(command_str: String) -> Command {
    let command_str: &str = &command_str;

    match command_str {
        "MoveLeft" => Command::MoveLeft,
        "MoveRight" => Command::MoveRight,
        "DasLeft" => Command::DasLeft,
        "DasRight" => Command::DasRight,
        "RotateCW" => Command::RotateCW,
        "RotateCCW" => Command::RotateCCW,
        "Rotate180" => Command::Rotate180,
        "HardDrop" => Command::HardDrop,
        "SoftDrop" => Command::SoftDrop,
        "HoldPiece" => Command::Hold,
        _ => Command::None,
    }
}

use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use tungstenite::{connect, Message};
use url::Url;

#[derive(Serialize, Deserialize)]
struct Information {
    contents: String,
}

pub fn human_play() {
    let mut human = Human::default();

    let empty_board = serde_json::to_string(&human.get_game_mut().get_board_json());
    let new_queue = serde_json::to_string(&human.get_game_mut().get_piece_queue_json());

    // Connect to the WS server locally
    let (mut socket, _response) =
        connect(Url::parse("ws://localhost:5678").unwrap()).expect("Can't connect");

    socket
        .write_message(Message::Text(new_queue.unwrap()))
        .unwrap();
    socket
        .write_message(Message::Text(empty_board.unwrap()))
        .unwrap();

    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            Message::Text(s) => s,
            _ => {
                panic!("Cannot connect to GUI. Consider running main.py")
            }
        };

        let parsed: Information = serde_json::from_str(&msg).expect("Can't parse to JSON");

        // update board
        human.set_next_move(parsed.contents);
        human.make_move();

        // send board
        let board_state = serde_json::to_string(&human.get_game_mut().get_board_json());
        let queue = serde_json::to_string(&human.get_game_mut().get_piece_queue_json());

        socket.write_message(Message::Text(queue.unwrap())).unwrap();

        if let Some(hold) = human.get_game_mut().hold_piece {
            let hold = serde_json::to_string(&json!({
                        "hold": PieceQueue::int_to_piece(hold),
                        "kind": String::from("hold")}));
            socket.write_message(Message::Text(hold.unwrap())).unwrap();
        }

        socket
            .write_message(Message::Text(board_state.unwrap()))
            .unwrap();
    }
}
