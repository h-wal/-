use std::collections::{BTreeMap, HashMap};
use tokio::sync::{mpsc, oneshot};
use crate::actors::db::{DbCommand, DbSender};
use crate::domain::{MarketBook, Order, Side};

pub enum OrderbookCommand {
    NewLimitOrder {
        market_id: u64,
        user_id: String,
        side: Side,
        qty: u64,
        price: u64,
        resp: oneshot::Sender<OrderbookResponse> 
    },
    NewMarketOrder {
        market_id: u64,
        user_id: String,
        side: Side,
        qty: u64,
        resp: oneshot::Sender<OrderbookResponse>
    },
    GetBook {
        market_id: u64,
        resp: oneshot::Sender<OrderbookResponse>
    }
}

pub struct OrderbookResponse {
    pub status: String,
    pub fills: Vec<crate::domain::Trade>,
    pub remaining_qty: u64,
    pub bids: Option<BTreeMap<u64, std::collections::VecDeque<Order>>>,
    pub asks: Option<BTreeMap<u64, std::collections::VecDeque<Order>>>,
}

pub async fn start_orderbook_actor(mut rx: mpsc::Receiver<OrderbookCommand>, db_tx: DbSender) {
    let mut order_book: HashMap<u64, MarketBook> = HashMap::new();

    let order_book_1 = MarketBook::new();
    order_book.insert(1, order_book_1);

    println!("MarketBookDbActor Started");
    println!("Initialized first market with id = 1");

    while let Some(cmd) = rx.recv().await {
        match cmd {
            OrderbookCommand::NewLimitOrder { market_id, user_id, side, qty, price, resp } => {
                let response = if order_book.contains_key(&market_id) {
                    let (oneshot_tx, oneshot_rx) = oneshot::channel();
                    let _ = db_tx.send(DbCommand::CheckUser {
                        user_email: user_id.clone(),
                        response_status: oneshot_tx 
                    })
                    .await;

                    match oneshot_rx.await {
                        Ok(response) => {
                            if response.user_exists {
                                println!("User exists");
                                println!("Market {} exists, inserting order...", market_id);

                                if let Some(book) = order_book.get_mut(&market_id) {
                                    let order = Order::new(user_id, qty, price, side);
                                    book.insert_order(order);
                                }

                                OrderbookResponse {
                                    status: "Order added Successfull".to_string(),
                                    fills: vec![],
                                    remaining_qty: 0,
                                    bids: None,
                                    asks: None
                                }
                            } else {
                                println!("User does not exists.");
                                OrderbookResponse {
                                    status: "User Does Not Exists".to_string(),
                                    fills: vec![],
                                    remaining_qty: 0,
                                    bids: None,
                                    asks: None
                                }
                            }
                        }
                        Err(_) => {
                            println!("Error finding User in the database");
                            OrderbookResponse {
                                status: "Error finding User in the data base".to_string(),
                                fills: vec![],
                                remaining_qty: 0,
                                bids: None,
                                asks: None
                            }
                        }
                    }
                } else {
                    println!("Market with Market id = {}, does not exist", market_id);
                    OrderbookResponse {
                        status: "Market does not exist".to_string(),
                        fills: vec![],
                        remaining_qty: 0,
                        bids: None,
                        asks: None
                    }
                };

                let _ = resp.send(response);
            }
            OrderbookCommand::NewMarketOrder { market_id, user_id: _, side: _, qty: _, resp } => {
                let response = if order_book.contains_key(&market_id) {
                    //Todo Create Market Order
                    println!("Market {} exists, inserting order...", market_id);
                    OrderbookResponse {
                        status: "Order added Successfull".to_string(),
                        fills: vec![],
                        remaining_qty: 0,
                        bids: None,
                        asks: None
                    }
                } else {
                    println!("Market with Market id = {}, does not exist", market_id);
                    OrderbookResponse {
                        status: "Market does not exist".to_string(),
                        fills: vec![],
                        remaining_qty: 0,
                        bids: None,
                        asks: None
                    }
                };
                let _ = resp.send(response);
            }
            OrderbookCommand::GetBook { market_id, resp } => {
                let response = if order_book.contains_key(&market_id) {
                    println!("Market Exists , id = {}", market_id);

                    if let Some(book) = order_book.get(&market_id) {
                        OrderbookResponse {
                            status: "Successfull! This is the current status of the orderBook".to_string(),
                            fills: vec![],
                            remaining_qty: 0,
                            bids: Some(book.bids.clone()),
                            asks: Some(book.asks.clone())
                        } 
                    } else {
                        OrderbookResponse {
                            status: "Cannot find the orderBook".to_string(),
                            fills: vec![],
                            remaining_qty: 0,
                            bids: None,
                            asks: None
                        }
                    }
                } else {
                    println!("Market does not exists, id = {}", market_id);
                    OrderbookResponse { 
                        status: "Market Does not exists".to_string(), 
                        fills: vec![], 
                        remaining_qty: 0, 
                        bids: None,
                        asks: None
                    }
                };
                let _ = resp.send(response);
            }
        }
    }
}

