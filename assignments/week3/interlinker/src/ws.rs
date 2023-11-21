use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc as tokio_mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{WebSocket, Message};
use futures::StreamExt;

use crate::client::{Clients, Client};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

pub async fn client_connection(ws: WebSocket, clients: Clients) {
    let client_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    println!("client {client_id} connected");

    let (client_ws_tx, mut client_ws_rx) = ws.split();
    let (client_tx, client_rx) = tokio_mpsc::unbounded_channel();
    let client_rx = UnboundedReceiverStream::new(client_rx);

    // We make rx (the receiving half of the unbounded channel) into a
    // stream and then spawn a task to forward that stream onto the
    // user's WebSocket.
    tokio::task::spawn(client_rx.forward(client_ws_tx));
    client_tx.send(Ok(Message::text("pong"))).unwrap();
    clients.write().await.insert(client_id, Client { sender: client_tx });

    while let Some(msg_result) = client_ws_rx.next().await {
        let msg = match msg_result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error from client={client_id}: {e:?}");
                break;
            }
        };

        println!("client {client_id} sent: {msg:?}");
        let _ = handle_msg(client_id, msg, &clients).await;
        // say something
    }

    clients.write().await.remove(&client_id);
    println!("client {client_id} disconnected");
}

use quick_xml::events::Event;
use quick_xml::reader::Reader;

async fn handle_msg(client_id: usize, msg: Message, clients: &Clients) -> Result<(), Box<dyn std::error::Error>> {
    if !msg.is_text() {
        Err("expected text message")?;
    }
    let url = msg.to_str().unwrap();
    println!("url: {url}");

    let client = reqwest::Client::new();
    let document = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.2 (KHTML, like Gecko) Chrome/22.0.1216.0 Safari/537.2")
        .send()
        .await?
        .text()
        .await?;

    let mut reader = Reader::from_str(&document);
    let reader = reader
        .check_end_names(false)
        .check_comments(false)
        .expand_empty_elements(false)
        .trim_text(true);

    let mut self_links = 0;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into_async(&mut buf).await {
            Ok(Event::Start(e)) => {
                let tag = e.name().into_inner();
                if tag != b"a" { continue };
                let href = e.html_attributes()
                    .find(|attr| attr.as_ref().is_ok_and(|attr| attr.key.0 == b"href"))
                    .map(|attr| String::from_utf8(attr.unwrap().value.to_vec()));

                if let Some(Ok(url)) = href {
                    println!("{tag:?} -> {url:?}");
                }
            },
            Ok(Event::Text(e)) => {},
            Err(e) => {
                println!("Error at position {}: {:?}", reader.buffer_position(), e)
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
