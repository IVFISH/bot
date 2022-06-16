mod board;
mod bot;
mod communications;
mod errors;
mod game;
mod human;
mod placement;
mod players;
mod queue;

use crate::bot::*;
use crate::human::*;
use crate::players::*;

use crate::game::Game;
use futures_util::{SinkExt, StreamExt};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::{Message, Result};

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
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    // Echo incoming WebSocket messages and send a message periodically every second.
    let mut bot = Bot::new(None, Default::default());

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                if msg.is_none() {
                    break
                }
                let msg = msg.unwrap().unwrap();

                if msg.is_close() {
                    break;
                }

                let parsed: serde_json::Value = serde_json::from_str(msg.to_text().unwrap()).unwrap();
                let parsed_type = parsed["type"].as_str().unwrap();

                match parsed_type {
                    "rules" => bot = create_bot_from_parsed(&parsed),
                    "play" => {
                        let converted: Vec<Vec<bool>> = parsed["board"].as_array().unwrap().iter().map(|wrappedvec: &serde_json::Value| {wrappedvec.as_array().unwrap().iter().map(|wrappedbool: &serde_json::Value| {wrappedbool.as_bool().unwrap()}).collect()}).collect();
                        bot.get_game().board.set_board(converted);},
                    "stop" => println!("stop game"),
                    "start" => println!("start game"),
                    other => eprintln!("unexpected packet of type {}", other),
                }

                println!("packet of type {} was recieved!", parsed["type"]);

                ws_sender.send(msg).await?;
            }
        }
        // interval.tick();
        // ws_sender.send(Message::Text("tick".to_owned())).await?;
    }
    Ok(())
}

fn create_bot_from_parsed(parsed: &serde_json::Value) -> Bot {
    Bot::create(Game::create(
        parsed["seed"].as_u64().unwrap() as usize,
        parsed["bagtype"].as_str().unwrap_or("singleplayer"),
        parsed["allow180"].as_bool().unwrap(),
        parsed["allow_harddrop"].as_bool().unwrap_or(true),
        parsed["b2bchaining"].as_bool().unwrap_or(true),
        parsed["boardheight"].as_u64().unwrap() as usize,
        parsed["kickset"].as_str().unwrap(),
        parsed["spinbonuses"].as_str().unwrap_or("singleplayer")))
}


#[tokio::main]
async fn main() {
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
