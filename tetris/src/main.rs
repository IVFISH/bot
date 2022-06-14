mod board;
mod placement;
mod errors;
mod game;
mod queue;
mod players;
mod bot;
mod human;

use crate::bot::*;
use crate::human::*;
use crate::players::*;

use crate::queue::PieceQueue;

use tungstenite::{connect, Message};
use url::Url;
use serde_json;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Information {
    contents: String,
}

fn main() {
    let mut human = Human::default();

    let empty = human.get_game().get_board_string();

    // Connect to the WS server locally
    let (mut socket, _response) = connect(Url::parse("ws://localhost:5678").unwrap()).expect("Can't connect");

    socket.write_message(Message::Text(human.get_game().get_piece_queue().to_string()).into()).unwrap();
    socket.write_message(Message::Text(empty).into()).unwrap();


    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };

        let parsed: Information = serde_json::from_str(&msg).expect("Can't parse to JSON");

        // update board
        human.set_next_move(parsed.contents);
        human.make_move();

        // send board
        let board_state = human.get_game().get_board_string();
        let queue = human.get_game().get_piece_queue();

        socket.write_message(Message::Text(queue.to_string()).into()).unwrap();

        if let Some(hold) = human.get_game().hold_piece {
            let hold = PieceQueue::int_to_piece(hold);
            socket.write_message(Message::Text(hold.into())).unwrap();
        }

        socket.write_message(Message::Text(board_state).into()).unwrap();
    }
}
