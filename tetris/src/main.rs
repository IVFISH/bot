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

use tungstenite::{connect, Message};
use url::Url;
use serde_json;
use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Eshan {
    eshan: String
}

fn main() {
    let empty = [[false; 10]; 23];

    // Connect to the WS server locally
    let (mut socket, _response) = connect(Url::parse("ws://localhost:5678").unwrap()).expect("Can't connect");

    let mut human = Human::default();
    println!("{}", human);
    socket.write_message(Message::Text(format!("{:?}", empty).into())).unwrap();
    socket.write_message(Message::Text(human.get_game().get_piece_queue().to_string()).into()).unwrap();



    // Loop forever, handling parsing each message
    loop {
        println!("reeee");
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: Eshan = serde_json::from_str(&msg).expect("Can't parse to JSON");
        println!("{}", parsed.eshan);

        // update board
        human.set_next_move(parsed.eshan);
        human.make_move();
        println!("{}", human);

        // send board
        let boardstate = human.get_game().get_board_array();
        let queue = human.get_game().get_piece_queue();
        socket.write_message(Message::Text(format!("{:?}", boardstate).into())).unwrap();
        socket.write_message(Message::Text(queue.to_string()).into()).unwrap();


    }
}

// fn main() {
//
//     let mut bot = Bot::default();
//
//     println!("{}", bot);
//     for _ in 0..15 {
//         bot.make_move();
//         println!("{}", bot);
//     }
//
// }
