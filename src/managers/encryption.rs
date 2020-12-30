use tokio::net::TcpStream;
use tokio::io::{BufReader, BufWriter, AsyncWriteExt};
use orion::aead::*;
use crate::core_structures::as_bytes::AsBytes;

pub struct EncryptionManager {
    key: SecretKey,
    pub stream: TcpStream
}


impl EncryptionManager {
    pub fn new(stream: TcpStream) -> EncryptionManager {
        let key = SecretKey::default();
        EncryptionManager {
            key,
            stream
        }
    }

    pub async fn init(&mut self) {
        let k = self.key.as_kv_bytes();
        let ciphertext = seal(&self.key, k.as_slice());
        if ciphertext.is_ok() {
            let mut writer = BufWriter::new(&mut self.stream);
            writer.write_all(ciphertext.unwrap().as_slice()).await;
            writer.flush().await;
        }
    }

    pub async fn write<A: AsBytes>(&mut self, data: &A) {
        let ciphertext = seal(&self.key, data.as_kv_bytes().as_slice());
        if ciphertext.is_ok() {
            let mut writer = BufWriter::new(&mut self.stream);
            writer.write_all(ciphertext.unwrap().as_slice()).await;
            writer.flush().await;
        }
    }
    // pub fn read(&mut self, reader: &mut BufReader<TcpStream>) {
    //     // let data = open(&self.key, )
    // }
}