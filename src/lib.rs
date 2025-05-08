use serde::{Deserialize, Serialize};

pub mod webview;

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

impl PeerConfig {
    pub fn new(
        id: &str,
        host: &str,
        port: &str,
        turn_host: &str,
        turn_port: &str,
        turn_username: &str,
        turn_credential: &str,
        secure: bool,
    ) -> Self {
        Self {
            id: id.to_string(),
            host: host.to_string(),
            port: port.to_string(),
            turn_host: turn_host.to_string(),
            turn_port: turn_port.to_string(),
            turn_username: turn_username.to_string(),
            turn_credential: turn_credential.to_string(),
            secure,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
