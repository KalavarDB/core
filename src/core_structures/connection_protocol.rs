use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;

#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Bytes,
    Struct,

}

#[derive(Debug, Clone)]
pub struct ConnectionProtocolMessage {
    pub recipient: usize,
    pub inner_type: DataType,
    pub inner: (Option<String>, Option<Box<[u8]>>/*, Option<Query>, Option<QueryResult>*/),
    pub opcode: OpCodes
}

impl ConnectionProtocolMessage {

}