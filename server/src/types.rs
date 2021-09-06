use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SocketEvent {
    pub event: String,
    pub data: String
}
