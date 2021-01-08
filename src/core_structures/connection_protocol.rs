// Internal crate imports
use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;

// An enum describing the types of data that a ConnectionProtocolMessage may contain
#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Bytes,
    MEM,
    Struct,
}

// A structure defined to help move information between threads via broadcast channels
#[derive(Debug, Clone)]
pub struct ConnectionProtocolMessage {
    // The intended recipient of the payload (0 for the protocol thread, or anything else if it is a client connection)
    pub recipient: usize,

    // The ID of the thread which sent the payload
    pub sender: usize,

    // The type that should be expected as present
    pub inner_type: DataType,

    // The inner data
    // A load of Option types with varying containers for different data structures as needed
    pub inner: (Option<String>, Option<Box<[u8]>>, Option<(f64, f64, f64)>/*, Option<Query>, Option<QueryResult>*/),

    // The OP code of the payload, see the docs at the link below for a breakdown:
    // https://kalavar.cf/documentation/general/opcodes
    pub opcode: OpCodes,
}

/// # The following content is undocumented due to not being ready for documentation at this time.
/// # You are welcome to attempt to make sense of it though.
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