use tokio::sync::mpsc;
use crate::app::{AppState, create_router};
use crate::actors::{start_db_actor, start_orderbook_actor, DbCommand, OrderbookCommand};

pub async fn run() {
    //? Starting the database actor
    let (db_tx, db_rx) = mpsc::channel::<DbCommand>(32);
    tokio::spawn(start_db_actor(db_rx));

    // Starting the orderbook actor
    let (ob_tx, ob_rx) = mpsc::channel::<OrderbookCommand>(32);
    tokio::spawn(start_orderbook_actor(ob_rx, db_tx.clone()));

    //Main state's of the Application for data trasnder between the 2 threads
    let state = AppState {
        db_tx: db_tx.clone(),
        ob_tx: ob_tx.clone(),
    };

    // Create router
    let app = create_router().with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .expect("Failed to bind to address");
    
    println!("Server running on http://0.0.0.0:4000");
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

