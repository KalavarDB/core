use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;

#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Bytes,
    MEM,
    Struct,
}

#[derive(Debug, Clone)]
pub struct ConnectionProtocolMessage {
    pub recipient: usize,
    pub sender: usize,
    pub inner_type: DataType,
    pub inner: (Option<String>, Option<Box<[u8]>>, Option<(f64, f64, f64)>/*, Option<Query>, Option<QueryResult>*/),
    pub opcode: OpCodes,
}

impl ConnectionProtocolMessage {
    pub fn new_mem(used: f64, total: f64) -> ConnectionProtocolMessage {
        ConnectionProtocolMessage {
            recipient: 0,
            sender: 0,
            inner_type: DataType::MEM,
            inner: (None, None, Some((used, total, (used / total * 100 as f64)))),
            opcode: OpCodes::MemUse,
        }
    }
    pub fn new_con(id:&usize ) -> ConnectionProtocolMessage {
        ConnectionProtocolMessage {
            recipient: 0,
            sender: *id,
            inner_type: DataType::String,
            inner: (None, None, None),
            opcode: OpCodes::Connect,
        }
    }
}