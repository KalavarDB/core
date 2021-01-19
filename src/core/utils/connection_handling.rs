use crate::managers::connections::connection::Connection;





pub mod api;

/// # The following content is undocumented due to not being ready for documentation at this time.
/// # You are welcome to attempt to make sense of it though.
pub async fn handle(mut _con: Connection) {
    // let broken = false;
    // let mut status = con.stream.stream.readable().await;
    // while (status.is_ok() || con.opened.elapsed().as_millis() < 1000) && !broken {
    //     con.logger.debug_message(format!("broken: {}", broken)).await;
    //     // let mut line = "\x1b[0m".to_string();
    //
    //     if broken {
    //         let mut m = ConnectionProtocolMessage::new_con(&con.id);
    //         m.opcode = OpCodes::Disconnect;
    //         con.transmitter.send(m);
    //         con.stream.stream.shutdown().await;
    //         break;
    //     }
    //
    //     status = con.stream.stream.readable().await;
    //     // let response = con.receiver.recv().await;
    //     // if response.is_ok() {
    //     //     con.logger.debug_pretty(response.unwrap()).await;
    //     //     //
    //     //     // let incoming = con.read_incoming().await;
    //     //     //
    //     //     // // con.transmitter.send(ConnectionProtocolMessage::pong());
    //     // } else {
    //     //     println!("error: {}", response.unwrap_err())
    //     // }
    // }
}