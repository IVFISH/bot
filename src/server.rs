use crate::constants::piece_constants::*;
use crate::piece::*;
use crate::suggestion::*;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{interval, Interval};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tungstenite::protocol::Message;
use tungstenite::{Error, Result};

async fn accept_connection<S>(stream: TcpStream) {
    let ws_stream = accept_async(stream).await.unwrap();
    let _ = handle_connection(ws_stream).await;
}

async fn handle_connection<S>(ws_stream: S) -> Result<()>
where
    S: Unpin + Stream<Item = Result<Message, Error>> + Sink<Message>,
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
                // get the board state, queue, etc. from msg
            }
            // branch 2: bot is ready to input to client
            inputs = get_suggestion(&mut interval) => {
                let msg = serde_json::to_string(&inputs).unwrap();
                ws_sender.send(Message::Text(msg));
            }
        }
    }
    Ok(())
}

/// gets the next message from the stream and processes the info for the bot
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
async fn get_suggestion(interval: &mut Interval) -> Suggestion {
    interval.tick().await;
    // dummy code
    let piece = Piece::new(PIECE_O);
    Suggestion::new(piece)
}
