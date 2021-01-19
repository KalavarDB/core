use tokio::net::TcpStream;
use tokio::io::{BufReader, BufWriter, AsyncWriteExt};
use orion::aead::*;
use crate::core_structures::as_bytes::AsBytes;

/// A utility structure for handling encryption between the server and the client
pub struct EncryptionManager {

    /// The key used to encrypt the data
    key: SecretKey,

    /// The stream the data should be sent to, and decrypted from
    pub stream: TcpStream
}


impl EncryptionManager {
    /// A utility method to instantiate a new encryption manager for a given TcpStream
    pub fn new(stream: TcpStream) -> EncryptionManager {
        EncryptionManager {
            key: SecretKey::default(),
            stream
        }
    }

    /// (OBSOLETE) A method to send information about the encryption methods used to the client
    pub async fn init(&mut self) {
        let k = self.key.as_kv_bytes();
        let ciphertext = seal(&self.key, k.as_slice());
        if ciphertext.is_ok() {
            let payload = ciphertext.unwrap();
            // println!("{:#?}", payload);
            println!("{} bytes", payload.len());

            let mut writer = BufWriter::new(&mut self.stream);
            writer.write_all(payload.as_slice()).await.unwrap();
            writer.flush().await.unwrap();
            println!("key transmitted");
        }
    }


    /// Encrypt and then write the provided data to the TcpStream and flush the buffer so that it is transmitted to the client for processing
    #[allow(dead_code)]
    pub async fn write<A: AsBytes>(&mut self, data: &A) {
        let ciphertext = seal(&self.key, data.as_kv_bytes().as_slice());
        if ciphertext.is_ok() {
            let mut writer = BufWriter::new(&mut self.stream);
            writer.write_all(ciphertext.unwrap().as_slice()).await.unwrap();
            writer.flush().await.unwrap();
        }
    }

    /// (OBSOLETE) Read and subsequently decrypt the data received from the client
    #[allow(dead_code)]
    pub fn read(&mut self, _reader: &mut BufReader<TcpStream>) {
        // let data = open(&self.key, )
    }
}