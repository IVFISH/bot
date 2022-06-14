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

    let empty_board = serde_json::to_string(&human.get_game().get_board_json());
    let empty_queue = serde_json::to_string(&human.get_game().get_piece_queue_json());


    // Connect to the WS server locally
    let (mut socket, _response) = connect(Url::parse("ws://localhost:5678").unwrap()).expect("Can't connect");

    socket.write_message(Message::Text(empty_queue.unwrap())).unwrap();
    socket.write_message(Message::Text(empty_board.unwrap())).unwrap();


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
        let board_state = serde_json::to_string(&human.get_game().get_board_json());
        let queue = serde_json::to_string(&human.get_game().get_piece_queue_json());

        socket.write_message(Message::Text(queue.unwrap())).unwrap();

        if let Some(hold) = human.get_game().hold_piece {
            let hold = serde_json::to_string(
                &json!({
                        "hold": PieceQueue::int_to_piece(hold),
                        "kind": String::from("hold")}));
            socket.write_message(Message::Text(hold.unwrap())).unwrap();
        }

        socket.write_message(Message::Text(board_state.unwrap())).unwrap();
    }
}
