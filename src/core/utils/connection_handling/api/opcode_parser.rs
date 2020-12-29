#[derive(Debug, Clone)]
pub enum OpCodes {
    Normal,          // 00
    Ping,            // 01
    Pong,            // 02
    Disconnect,      // 03
    Reconnect,       // 04
    Shutdown,        // 05
    Status,          // 06
    Hello,           // 07
    Authenticate,    // 09
    Ready,           // 10
    // Reserved      // 11
    // Reserved      // 12
    // Reserved      // 13
    // Reserved      // 14
    // Reserved      // 15
    // Reserved      // 16
    // Reserved      // 17
    // Reserved      // 18
    Terminate,       // 19
    PayloadTamper,   // 20
}