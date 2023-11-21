use std::{sync::Arc, collections::HashMap};
use tokio::sync::{mpsc, RwLock};
use warp::filters::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub sender: mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>,
}

pub type Clients = Arc<RwLock<HashMap<usize, Client>>>;
