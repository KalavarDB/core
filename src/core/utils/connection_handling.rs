use crate::managers::connections::connection::Connection;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

use tokio::io::{Interest, AsyncBufReadExt, BufReader, Ready, AsyncWriteExt};
use crate::errors::ErrorMap::EXXX;
use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;
use tokio::runtime::Handle;
use std::process::exit;

pub mod api;

pub async fn handle(mut con: Connection) {
    let mut broken = false;
    let mut status = con.stream.stream.readable().await;
    while (status.is_ok() || con.opened.elapsed().as_millis() < 1000) && !broken {
        con.logger.debug_message(format!("broken: {}", broken)).await;
        // let mut line = "\x1b[0m".to_string();



        if broken {
            let mut m = ConnectionProtocolMessage::new_con(&con.id);
            m.opcode = OpCodes::Disconnect;
            con.transmitter.send(m);
            con.stream.stream.shutdown().await;
            break;
        }

        status = con.stream.stream.readable().await;
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
    }
}