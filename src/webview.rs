use std::{env::temp_dir, fs::File, io::Write, sync::Arc};

use anyhow::{bail, Result};
use serde::Deserialize;
use tokio::sync::{mpsc, Mutex};
use warp::Filter;

pub async fn run_webview() -> Result<String> {
    std::thread::spawn(|| {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("temp_index.html");

        let content = include_str!("../client.html");

        match File::create(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    println!("❌ Failed to write to file: {}", e);
                }

                let url = format!("file:///{}", file_path.to_str().unwrap().replace('\\', "/"));
                match webbrowser::open(&url) {
                    Ok(_) => println!("✅ Opened webview at: {}", url),
                    Err(e) => eprintln!("❌ Failed to open webview: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Failed to create file: {}", e);
                return;
            }
        }
    });
    let (tx, mut rx) = mpsc::channel::<String>(1);
    let tx_arc = Arc::new(Mutex::new(tx));
    let cors = warp::cors()
        .allow_any_origin()
        .allow_method(warp::http::Method::POST)
        .allow_header("content-type");

    let route = warp::post()
        .and(warp::path("peer-id"))
        .and(warp::body::json())
        .map(move |payload: PeerIdPayload| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let _ = tx_clone.lock().await.send(payload.id.clone()).await;
            });
            warp::reply()
        })
        .with(cors);

    tokio::spawn(warp::serve(route).run(([127, 0, 0, 1], 8654)));

    match rx.recv().await {
        Some(id) => {
            println!("✅ Received peer ID: {}", id);
            Ok(id)
        }
        None => {
            bail!("❌ Failed to receive peer ID")
        }
    }
}

#[derive(Deserialize)]
struct PeerIdPayload {
    id: String,
}
