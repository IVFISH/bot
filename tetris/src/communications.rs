#![allow(dead_code)]

use crate::bot::*;
use crate::game::Game;
use crate::players::*;
use crate::queue::piece_type_to_string;

use crate::constants::types::PieceType;
use crate::game::game_rules_and_data::GameRules;
use crate::Piece;
use futures_util::{SinkExt, StreamExt};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::{thread, time};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::{Message, Result};

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub input_list: Vec<String>,
    pub info: String,
}

impl Suggestion {
    pub fn new(input_list: Vec<String>, info: String) -> Self {
        Self { input_list, info }
    }
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {:?}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    // let interval = tokio::time::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.
    let mut bot = Bot::default();
    bot.get_game_mut().hard_drop();

    loop {
        tokio::select! {
            Some(msg) = ws_receiver.next() => {
                let msg = msg.unwrap();

                if msg.is_close() {
                    break;
                }

                let parsed: serde_json::Value = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                let parsed_type = parsed["type"].as_str().unwrap();

                match parsed_type {
                    "rules" => {
                        eprintln!("start game");
                        bot = create_bot_from_parsed(&parsed)
                    },
                    "play" => {

                        // Set bot board to tetrio board
                        for (r_index, row) in parsed["board"].as_array().unwrap().iter().rev().enumerate() {
                            bot.get_game_mut().board.set_row(r_index, row.as_array().unwrap().iter().map(|col| col.as_bool().unwrap()).collect())
                        }

                        // Error Correction
                        let tetrio_piece = str_to_piece_type(parsed["current"].as_str().unwrap());
                        let bot_piece = bot.get_game().active_piece.piece_type;
                        if tetrio_piece != bot_piece {
                            eprintln!(
                                "Active Piece Desynched: expected {}, but recieved {} instead",
                                piece_to_string(Some(bot_piece)),
                                piece_to_string(Some(tetrio_piece)));
                        }

                        let mut tetrio_queue: VecDeque<PieceType> = VecDeque::new();
                        let bot_queue = bot.get_game().piece_queue.get_queue();

                        for piece in parsed["queue"].as_array().unwrap(){
                            tetrio_queue.push_back(str_to_piece_type(piece.as_str().unwrap()))
                        }

                        if &tetrio_queue != bot_queue {
                            eprintln!("Mismatched Queue: expected {}, but received {} instead", bot.get_game().piece_queue, parsed["queue"]);
                            bot.get_game_mut().piece_queue.set_queue(tetrio_queue);
                        }

                        let bot_hold = piece_to_string(bot.get_game().get_hold_piece());
                        let tetrio_hold = parsed["hold"].as_str().unwrap_or_else(|| "*");
                        if bot_hold != tetrio_hold{
                             eprintln!("Mismatched Hold: expected {} as hold, received {} instead", bot_hold
                            , tetrio_hold);
                            bot.get_game_mut().hold_piece = str_to_piece(tetrio_hold);
                        }

                        // Primitive speedcap
                        thread::sleep(time::Duration::from_millis(0));
                        // Calculate and send move
                        ws_sender.send(Message::Text(serde_json::to_string(&json!(bot.make_suggest_move())).unwrap())).await?;
                    },
                    "stop" => eprintln!("stop game"),
                    "start" => {
                        ws_sender.send(Message::Text(serde_json::to_string(&json!(bot.make_suggest_move())).unwrap())).await?
                    },
                    other => eprintln!("unexpected packet of type {}", other),
                }

                fn str_to_piece(piece: &str) -> Option<usize> {
                    match piece {
                                "z" => Some(0),
                                "l" => Some(1),
                                "o" => Some(2),
                                "s" => Some(3),
                                "i" => Some(4),
                                "j" => Some(5),
                                "t" => Some(6),
                                _ => None,
                            }
                }

                fn str_to_piece_type(piece: &str) -> PieceType {
                    match piece {
                                "z" => 0,
                                "l" => 1,
                                "o" => 2,
                                "s" => 3,
                                "i" => 4,
                                "j" => 5,
                                "t" => 6,
                                _ => panic!(),
                            }
                }

                fn piece_to_string(piece: Option<usize>) -> &'static str {
                    match piece {
                                Some(0) => "z",
                                Some(1) => "l",
                                Some(2) => "o",
                                Some(3) => "s",
                                Some(4) => "i",
                                Some(5) => "j",
                                Some(6) => "t",
                                None => "*",
                                Some(_) => "?"
                            }
                }

                // eprintln!("packet of type {} was recieved!", parsed["type"]);

                // ws_sender.send(msg).await?; Echo response back to client
            }
        }
    }
    Ok(())
}

fn create_bot_from_parsed(parsed: &serde_json::Value) -> Bot {
    Bot::new(Game::from_rules(
        Some(parsed["seed"].as_u64().unwrap() as usize),
        GameRules {
            bag_type: parsed["bagtype"]
                .as_str()
                .unwrap_or("singleplayer")
                .parse()
                .unwrap(),
            allow_hard_drop: parsed["allow_harddrop"].as_bool().unwrap_or(true),
            allow_180: parsed["allow180"].as_bool().unwrap(),
            allow_b2b_chain: parsed["b2bchaining"].as_bool().unwrap_or(true),
            max_board_height: parsed["boardheight"].as_u64().unwrap() as usize,
            kick_set: parsed["kickset"].as_str().unwrap().parse().unwrap(),
            spin_bonus: parsed["spinbonuses"]
                .as_str()
                .unwrap_or("singleplayer")
                .parse()
                .unwrap(),
        },
    ))
}

#[tokio::main]
pub async fn init() {
    // env_logger::init();

    let addr = "127.0.0.1:23512";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}
