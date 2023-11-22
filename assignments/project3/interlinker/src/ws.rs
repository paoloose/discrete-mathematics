use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::mpsc as tokio_mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::filters::ws::{WebSocket, Message};
use futures::StreamExt;
use serde::{Serialize, Deserialize};

use crate::client::{Clients, Client};
use crate::utils::{is_valid_url, extract_domain};

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.2 (KHTML, like Gecko) Chrome/22.0.1216.0 Safari/537.2";

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
    let client = Client {
        id: client_id,
        sender: client_tx.clone(),
        active_origins: Default::default(),
    };
    clients.write().await.insert(client_id, client.clone());

    while let Some(msg_result) = client_ws_rx.next().await {
        let msg = match msg_result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("[ERR] from client={client_id}: {e:?}");
                break;
            }
        };

        let mut client_clone = client.clone();
        let clients_clone = clients.clone();

        tokio::spawn(async move {
            match handle_msg(&mut client_clone, msg, &clients_clone).await {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("[ERR] while handling: {e}");
                }
            }
        });
    }

    clients.write().await.remove(&client_id);
    println!("client {client_id} disconnected");
}

use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug, Deserialize)]
struct ClientRequest {
    subject: String,
    payload: String
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum ResponseMessage {
    FinishMessage {
        origin: String
    },
    ResultMessage {
        for_url: String,
        for_domain: String,
        linked_url: String,
        linked_domain: String,
    },
}

async fn handle_msg(client: &mut Client, msg: Message, _clients: &Clients) -> Result<(), Box<dyn std::error::Error>> {
    println!("[MSG] url: {msg:?}");
    if !msg.is_text() {
        Err("expected text message")?;
    }
    let request = serde_json::from_str::<ClientRequest>(msg.to_str().unwrap())?;

    if request.subject == "stop-origin" {
        let origin = request.payload;
        println!("[MSG] stop: {origin}");
        client.active_origins.write().await.retain(|x| *x != origin);
        return Ok(());
    }

    let origin = request.payload;
    let mut visited_domains: HashSet<String> = HashSet::new();

    let mut url_stack: HashSet<String> = HashSet::new();
    url_stack.insert(origin.clone());

    client.active_origins.write().await.push(origin.clone());

    loop {
        if url_stack.is_empty() {
            break;
        }
        let url = url_stack.iter().next().unwrap().clone();
        let url_domain = match extract_domain(url.as_str()) {
            Some(domain) => {
                url_stack.remove(&url);
                domain
            },
            None => {
                url_stack.remove(&url);
                continue;
            }
        };
        visited_domains.insert(url_domain.clone());

        let req = reqwest::Client::new();
        // but request can fail for other reasons so we still need to handle errors
        let response = req.get(url.clone())
            .header("user-agent", USER_AGENT)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await;

        if response.is_err() {
            continue;
        }
        let response = response.unwrap();

        match response.headers().get("content-type") {
            Some(content_type) => {
                if !content_type.to_str().unwrap().contains("text/html") {
                    continue;
                }
            },
            None => {
                continue;
            }
        }
        let document = response.text().await;
        if document.is_err() {
            continue;
        }
        let document = document.unwrap();

        // Build the XML reader
        let mut reader = Reader::from_str(&document);
        let reader = reader
            .check_end_names(false)
            .check_comments(false)
            .expand_empty_elements(false)
            .trim_text(true);

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event_into_async(&mut buf).await {
                Ok(Event::Start(e)) => {
                    println!("- {e:?}");
                    let tag = e.name().into_inner();
                    if tag != b"a" { continue; }
                    let attrs = e.html_attributes()
                        .find(|attr| attr.as_ref().is_ok_and(|attr| attr.key.into_inner() == b"href"))
                        .map(|attr| String::from_utf8(attr.unwrap().value.to_vec()).unwrap());

                    let href = match attrs {
                        Some(href) => {
                            if !href.starts_with("http") || !is_valid_url(href.as_str()) {
                                continue;
                            }
                            href
                        },
                        None => continue,
                    };

                    let domain_to_visit = match extract_domain(&href) {
                        Some(domain) => domain,
                        None => continue,
                    };

                    if !visited_domains.contains(&domain_to_visit) {
                        url_stack.insert(href.clone());
                        client.sender.send(Ok(
                            Message::text(serde_json::to_string(&ResponseMessage::ResultMessage {
                                for_url: url.clone(),
                                for_domain: url_domain.clone(),
                                linked_url: href.clone(),
                                linked_domain: domain_to_visit.clone(),
                            })?)
                        ))?;
                    }
                },
                Err(_) => {},
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }
        if client.active_origins.read().await.iter().find(|x| **x == origin).is_none() {
            break;
        }
    }

    client.sender.send(Ok(
        Message::text(serde_json::to_string(&ResponseMessage::FinishMessage {
            origin: origin.clone()
        })?)
    ))?;

    client.active_origins.write().await.retain(|x| *x != origin);

    Ok(())
}
