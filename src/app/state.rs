use crate::actors::{DbSender, OrderbookCommand};
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct AppState {
    pub db_tx: DbSender,
    pub ob_tx: mpsc::Sender<OrderbookCommand>,
}
