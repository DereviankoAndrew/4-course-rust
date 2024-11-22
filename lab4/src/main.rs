// Cargo.toml
// [dependencies]
// rocket = { version = "0.5.0-rc.2", features = ["json"] }
// tokio = { version = "1", features = ["full"] }
// tokio-tungstenite = "0.17"
// serde = { version = "1.0", features = ["derive"] }
// sled = "0.34"
// uuid = { version = "1", features = ["v4"] }
// serde_json = "1.0"
// tokio-stream = "0.1"

use rocket::{get, post, routes, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use tokio_stream::StreamExt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MessagePayload {
    sender: String,
    recipient: String,
    content: String,
}

type SharedDb = Arc<Mutex<Db>>;
type SharedTx = broadcast::Sender<MessagePayload>;

#[post("/register")]
async fn register(user: Json<User>, db: &State<SharedDb>) -> &'static str {
    let db = db.lock().unwrap();
    let key = format!("user:{}", user.username);
    if db.contains_key(&key).unwrap() {
        return "User already exists";
    }
    db.insert(key, user.password.as_bytes()).unwrap();
    "User registered"
}

#[post("/login")]
async fn login(user: Json<User>, db: &State<SharedDb>) -> &'static str {
    let db = db.lock().unwrap();
    let key = format!("user:{}", user.username);
    if let Some(stored_password) = db.get(&key).unwrap() {
        if stored_password.as_ref() == user.password.as_bytes() {
            return "Login successful";
        }
    }
    "Invalid username or password"
}

#[get("/history/<user>")]
async fn history(user: String, db: &State<SharedDb>) -> Json<Vec<MessagePayload>> {
    let db = db.lock().unwrap();
    let mut messages = vec![];
    for message in db.iter() {
        let (_, value) = message.unwrap();
        let payload: MessagePayload = serde_json::from_slice(&value).unwrap();
        if payload.sender == user || payload.recipient == user {
            messages.push(payload);
        }
    }
    Json(messages)
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Arc::new(Mutex::new(sled::open("chat_db")?));
    let (tx, _rx) = broadcast::channel(100);

    let db_clone = db.clone();
    let tx_clone = tx.clone();

    tokio::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();

        while let Ok((stream, _)) = listener.accept().await {
            let tx = tx_clone.clone();
            let rx = tx.subscribe();

            tokio::spawn(async move {
                let ws_stream = accept_async(stream).await.unwrap();
                let (mut write, mut read) = ws_stream.split();

                let rx_task = tokio::spawn(async move {
                    let mut rx = rx;
                    while let Ok(message) = rx.recv().await {
                        let msg = serde_json::to_string(&message).unwrap();
                        if write.send(Message::Text(msg)).await.is_err() {
                            break;
                        }
                    }
                });

                while let Some(Ok(Message::Text(text))) = read.next().await {
                    if let Ok(payload) = serde_json::from_str::<MessagePayload>(&text) {
                        tx.send(payload.clone()).unwrap();
                        let db = db_clone.lock().unwrap();
                        db.insert(
                            format!("msg:{}", Uuid::new_v4()),
                            serde_json::to_vec(&payload).unwrap(),
                        )
                        .unwrap();
                    }
                }

                rx_task.abort();
            });
        }
    });

    rocket::build()
        .manage(db)
        .manage(tx)
        .mount("/api", routes![register, login, history])
        .launch()
        .await?;

    Ok(())
}
