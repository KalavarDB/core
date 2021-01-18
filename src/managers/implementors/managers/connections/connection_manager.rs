use crate::core::utils::connection_handling::api::opcode_parser::OpCodes::*;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::errors::ErrorMap::*;
use crate::managers::{
    config::ConfigManager, connections::connection::Connection,
    connections::connection_manager::ConnectionManager, logging::LoggingManager,
    storage::StorageManager,
};

use jemalloc_ctl::{epoch, stats};
use tokio::net::TcpListener;
use tokio::sync::broadcast::{channel, Receiver, Sender};
use tokio::time::Duration;

impl ConnectionManager {
    /// Helper function used to instantiate a new connection manager on behalf of a caller
    pub async fn new(
        l: &mut LoggingManager,
        config: &ConfigManager,
        os: &str,
    ) -> ConnectionManager {
        // Directly return an initialized connection manager
        ConnectionManager {
            listener: None,
            port: config.config.network.bind_port,
            addr: config.config.network.bind_addr.clone(),
            dbm: StorageManager::new(l, os).await,
            connections: 0,
        }
    }

    /// Launches the memory management thread, and the storage management thread, whilst also beginning to listen for incoming TCP connections based off of the configuration options provided by the config manager
    pub async fn connect(&mut self, logger: &LoggingManager) {
        // Define a defualt transmitter and receiver which will be used for inter-thread communications across this codebase
        let (transmitter, mut receiver): (
            Sender<ConnectionProtocolMessage>,
            Receiver<ConnectionProtocolMessage>,
        ) = channel(100);

        logger
            .debug_message(format!("Binding to {}:{}", &self.addr, &self.port))
            .await;

        // Attempt to bind a TCP listener to the specified IP address and port number
        let bind_result = TcpListener::bind(format!("{}:{}", &self.addr, &self.port)).await;

        // If the bind was successful, begin the server's core functionality
        // If not, exit with a fatal error using code GXXX
        if bind_result.is_ok() {
            // The protocol handler
            // Define a new logging manager for use within the protocol handler thread
            let proclog = logger.clone();

            // Spawn a new dedicated thread for handling queries between the client and server as well as managing the storage system
            tokio::spawn(async move {
                // begin an endless loop of checking for incoming data from the connection pool, or memory manager
                loop {
                    // attempt to read the incoming data into
                    let incoming = receiver.recv().await;

                    // If the data was read successfully, process it accordingly
                    // If not throw a GXXX error, and log the error message to the terminal
                    if incoming.is_ok() {
                        let payload = incoming.unwrap();
                        // Verify that the protocol manager is the intended recipient of this broadcast
                        if payload.recipient == 0 {
                            // check the opcode against a logic tree of the actions to perform in the presence of each opcode
                            match payload.opcode {
                                MemUse => {
                                    //     let used = payload.inner.2.unwrap().2;
                                    //     proclog
                                    //         .debug_message(format!("Memory usage: {:.1$}%", used, 3))
                                    //         .await;
                                }
                                Disconnect => {}
                                Connect => {}
                                _ => {
                                    proclog.debug_pretty(&payload).await;
                                }
                            }
                        } else {
                            //Payload is not for the protocol handler, thus should be ignored
                        }
                    } else {
                        proclog.error(incoming.unwrap_err(), GXXX).await;
                    }
                }
            });

            // The memory manager
            // This manager reports the memory usage vs the memory available to the protocol thread, so it can make a judgement on
            // The state of the memory allocator, and the storage manager, and conduct itself accordingly

            // Create a transmitter for the memory management thread
            let memtra: Sender<ConnectionProtocolMessage> = transmitter.clone();

            // Spawn a thread that conntinuall checks the memory usage values of the process, and reports them to the protocol thread
            tokio::spawn(async move {
                // Create an endless loop
                loop {
                    // Update memory values
                    epoch::advance();

                    // Get the amount of RAM the process is using at the current time as a floating point value
                    let used = stats::allocated::read().unwrap() as f64;

                    // Get the total amount of RAM the process currently has assigned to it as a floating point value
                    let total = stats::resident::read().unwrap() as f64;

                    // Report the used and total values to the protocol thread for action
                    memtra.send(ConnectionProtocolMessage::new_mem(used, total));

                    // Sleep the whole thread for 20 seconds to avoid cpu usage spikes
                    tokio::time::sleep(Duration::from_secs(20)).await;
                }
            });
            
            // Unwrap the connection listener
            let incoming_connections = bind_result.unwrap();

            // Start an endless loop
            loop {
                // Create a transmitter for the new connection
                let temp_transmitter: Sender<ConnectionProtocolMessage> = transmitter.clone();

                // Create a receiver for the new connection
                let temp_receiver: Receiver<ConnectionProtocolMessage> = transmitter.subscribe();

                // Up the ID of the connection
                self.connections += 1;

                // Create a connection wrapper around the TCP stream to handle the basics of the protocol
                let mut connection = Connection::new(
                    self.connections,
                    &logger,
                    incoming_connections.accept().await.unwrap(),
                    temp_receiver,
                    temp_transmitter,
                ).await;

                // Spawn a thread to handle the connection and any requests it decides to make
                tokio::spawn(async move {
                    connection.transmitter.send(ConnectionProtocolMessage::new_con(&connection.id));
                    connection.stream.init().await;
                    // crate::core::utils::connection_handling::handle(connection).await
                });
            }
        } else {
            logger.fatal(bind_result.unwrap_err(), GXXX, 1).await;
        }
    }
}
