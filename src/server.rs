use crate::constants::piece_constants::*;
use crate::piece::*;
use crate::suggestion::*;
use crate::bot::*;
use crate::pruner::*;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{interval, Interval};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::protocol::Message;
use tungstenite::{Error, Result};

async fn accept_connection(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let bot = Bot::<AllClearPruner>::new(); 
    let _ = handle_connection(ws_stream, bot).await;
}

async fn handle_connection<S, P>(ws_stream: S, mut bot: Bot<P>) -> Result<()>
where
    S: Unpin + Stream<Item = Result<Message, Error>> + Sink<Message>,
    P: Pruner + std::marker::Sync,
{
    // split into a sink and a stream
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = interval(Duration::from_millis(1000)); // 1 pps
    loop {
        // waiting for multiple async branches
        tokio::select! {
            // branch 1: receiving updates from client
            msg = poll_next(&mut ws_receiver) => {
                if let Err(_) = msg {
                    break;
                }
                // idk what updates are here yet
            }
            // branch 2: bot makes move and tells client the move
            inputs = get_suggestion(&mut interval, &mut bot) => {
                let msg = serde_json::to_string(&inputs).unwrap();
                let _ = ws_sender.send(Message::Text(msg)).await;
            }
        }
    }
    Ok(())
}

/// gets the next message from the stream and updates the bot
async fn poll_next<S>(ws_receiver: &mut S) -> Result<(), Error>
where
    S: Stream<Item = Result<Message, Error>> + Unpin,
{
    match ws_receiver.next().await {
        Some(msg) => {
            let msg = msg?;
            if msg.is_text() || msg.is_binary() {
                Ok(())
            } else if msg.is_close() {
                Err(Error::ConnectionClosed)
            } else {
                // idk what this branch is lmao
                Ok(())
            }
        }
        None => Err(Error::ConnectionClosed),
    }
}

/// gets the next inputs from the bot
async fn get_suggestion<P>(interval: &mut Interval, bot: &mut Bot<P>) -> Suggestion
where P: Pruner + std::marker::Sync {
    let _ = interval.tick().await;
    bot.r#do()
}

/// driver function
#[tokio::main]
pub async fn init() {
    let addr = "127.0.0.1:23512";
    let listener = TcpListener::bind(&addr).await.unwrap();

    // accept a client connection
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}
