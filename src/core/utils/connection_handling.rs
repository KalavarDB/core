use crate::managers::connections::connection::Connection;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;

use tokio::io::{Interest, AsyncBufReadExt, BufReader, Ready, AsyncWriteExt};
use crate::errors::ErrorMap::EXXX;
use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;
use tokio::runtime::Handle;

pub mod api;

pub async fn handle(mut con: Connection) {
    let mut broken = false;
    let mut status = con.stream.readable().await;
    while (status.is_ok() || con.opened.elapsed().as_millis() < 1000) && !broken {
        con.logger.debug_message(format!("broken: {}", broken)).await;
        let mut line = "\x1b[0m".to_string();
        let mut lines = Vec::<String>::new();
        let mut read = BufReader::new(&mut con.stream);
        while line.as_bytes().len() != 0 && !broken {
            let _ = read.read_line(&mut line).await;
            con.logger.debug_message(format!("Should break: {}", line == "\x1b[0m".to_string())).await;
            if line == "\x1b[0m".to_string() {
                con.logger.debug_message("Breaking...").await;
                broken = true;
                break;
            }
            lines.push(line.clone());
            line = "\x1b[0m".to_string();
        }

        if broken {
            let mut m = ConnectionProtocolMessage::new_con(&con.id);
            m.opcode = OpCodes::Disconnect;
            con.transmitter.send(m);
            con.stream.shutdown();
            break;
        }

        status = con.stream.readable().await;
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