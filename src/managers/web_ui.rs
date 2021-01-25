use crate::managers::logging::LoggingManager;

use tokio::sync::broadcast::{Sender, Receiver};
use crate::managers::web_ui::routes::map_routes;

/// Documentation on the routes of the web ui;
pub(crate) mod routes;

/// Enumerator to move data between the analytics thread and the web UI management thread
#[derive(Debug, Clone)]
pub enum WebUIMessage {
    Open,
    Ready,
    Launch,
    Connection,
    Disconnect,
    Management,
}