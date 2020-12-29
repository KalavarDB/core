use crate::managers::connections::connection::Connection;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

pub mod api;

pub async fn handle(mut con: Connection) {
    loop {
        let response = con.receiver.recv().await;
        if response.is_ok() {
            let incoming = con.read_incoming().await;
            // con.transmitter.send(ConnectionProtocolMessage::pong());
        } else {
            println!("error: {}", response.unwrap_err())
        }
    }
}