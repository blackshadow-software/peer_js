use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerConfig {
    pub id: String,
    pub host: String,
    pub port: String,
    pub turn_host: String,
    pub turn_port: String,
    pub turn_username: String,
    pub turn_credential: String,
    pub secure: bool,
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
