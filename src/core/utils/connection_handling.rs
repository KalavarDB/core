use crate::managers::connections::connection::Connection;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

use tokio::io::{Interest, AsyncBufReadExt, BufReader};
use crate::errors::ErrorMap::EXXX;

pub mod api;

pub async fn handle(mut con: Connection) {
    loop {
        let status = con.stream.ready(Interest::READABLE | Interest::WRITABLE).await;
        if status.is_ok() {
            con.logger.debug( status.unwrap().is_write_closed()).await;
            let mut line = String::new();
            let mut read = BufReader::new(&mut con.stream);
            let _ = read.read_line(&mut line).await;
            con.logger.debug(line).await;
            // let response = con.receiver.recv().await;
            // if response.is_ok() {
            //     con.logger.debug_pretty(response.unwrap()).await;
            //     //
            //     // let incoming = con.read_incoming().await;
            //     //
            //     // // con.transmitter.send(ConnectionProtocolMessage::pong());
            // } else {
            //     println!("error: {}", response.unwrap_err())
            // }
        } else {
            con.logger.error( status.unwrap_err(), EXXX).await;

        }
    }
}