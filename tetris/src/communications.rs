#![allow(dead_code)]

use crate::bot::*;
use crate::game::Game;
use crate::players::*;
use crate::queue::piece_type_to_string;

use futures_util::{SinkExt, StreamExt};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::{net::SocketAddr};
use std::{thread, time};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::{Message, Result};
use crate::game::game_rules_and_data::GameRules;

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
                    "rules" => bot = create_bot_from_parsed(&parsed),
                    "play" => {
                        for (r_index, row) in parsed["board"].as_array().unwrap().iter().enumerate() {
                            bot.get_game_mut().board.set_row(r_index, row.as_array().unwrap().iter().map(|col| col.as_bool().unwrap()).collect())
                        }

                        // println!("local: {}, client: {}", bot.get_game().game_data.pieces_placed, parsed["placed"]);
                        if bot.get_game().game_data.pieces_placed != parsed["placed"] {
                            eprintln!("DESYNCED!!! Bot is has placed {} pieces, but client has placed {} pieces!",
                            bot.get_game().game_data.pieces_placed, parsed["placed"]);
                        }
                        for i in 0..6 {
                            if piece_type_to_string(bot.get_game().piece_queue.peek_index(i)) != parsed["queue"].as_array().unwrap()[i].as_str().unwrap() {
                                eprintln!("Mismatched Queue: expected {}, but received {} instead", bot.get_game().piece_queue, parsed["queue"]);
                                break;
                            }
                            // if (parsed["queue"].as_array().unwrap()[i].as_str().unwrap())
                        }
                        let current_hold = piece_to_string(bot.get_game().get_hold_piece());

                        if current_hold != parsed["hold"].as_str().unwrap_or_else(|| "*") {
                             eprintln!("Mismatched Hold: expected {} as hold, received {} instead", current_hold
                            , parsed["hold"].as_str().unwrap_or_else(|| "*"));
                        }

                        // println!("{}", bot);

                        thread::sleep(time::Duration::from_millis(0));
                        ws_sender.send(Message::Text(serde_json::to_string(&json!(bot.suggest_and_move())).unwrap())).await?;
                    },
                    "stop" => println!("stop game"),
                    "start" => ws_sender.send(Message::Text(serde_json::to_string(&json!(bot.suggest_and_move())).unwrap())).await?,
                    other => eprintln!("unexpected packet of type {}", other),
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
    Bot::new(Game::from_rules(Some(parsed["seed"].as_u64().unwrap() as usize),
                              GameRules {
                                  bag_type: parsed["bagtype"].as_str().unwrap_or("singleplayer").parse().unwrap(),
                                  allow_hard_drop: parsed["allow_harddrop"].as_bool().unwrap_or(true),
                                  allow_180: parsed["allow180"].as_bool().unwrap(),
                                  allow_b2b_chain: parsed["b2bchaining"].as_bool().unwrap_or(true),
                                  max_board_height: parsed["boardheight"].as_u64().unwrap() as usize,
                                  kick_set: parsed["kickset"].as_str().unwrap().parse().unwrap(),
                                  spin_bonus: parsed["spinbonuses"].as_str().unwrap_or("singleplayer").parse().unwrap(),
                              }))
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
