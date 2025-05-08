use std::{env::temp_dir, fs::File, io::Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

pub async fn run_webview(payload: &str) -> Result<()> {
    let config: PeerConfig = serde_json::from_str(payload).expect("Error deserializing JSON");

    std::thread::spawn(move || {
        let temp_dir = temp_dir();
        let file_path = temp_dir.join("temp_index.html");

        let content = include_str!("../client.html");
        // file:///D:/peerjs_app/client.html?peerId=userid&host=23.98.93.20&port=9000&turnHost=turn.example.com&turnPort=3478&turnUsername=testuser&turnCredential=testpass&end=end
        match File::create(&file_path) {
            Ok(mut file) => {
                if let Err(e) = file.write_all(content.as_bytes()) {
                    println!("❌ Failed to write to file: {}", e);
                }

                let base_url =
                    format!("file:///{}", file_path.to_str().unwrap().replace('\\', "/"));
                let params =format!("?peerId={}&host={}&port={}&turnHost={}&turnPort={}&turnUsername={}&turnCredential={}&end=end",config.id,config.host,config.port,config.turn_host,config.turn_port,config.turn_username,config.turn_credential);
                let url = format!("{}{}", base_url, params);
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

    Ok(())
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerConfig {
    id: String,
    host: String,
    port: String,
    turn_host: String,
    turn_port: String,
    turn_username: String,
    turn_credential: String,
    secure: bool,
}
