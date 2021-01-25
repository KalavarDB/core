use crate::managers::analytics::AnalyticsManager;
use crate::managers::config::post::PrivacyMode;
use crate::managers::logging::LoggingManager;
use crate::managers::web_ui::routes::map_routes;
use crate::managers::web_ui::WebUIMessage;

use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use tokio::sync::broadcast::{Receiver, channel, Sender};
use tokio::time::{Duration, Instant};
use tokio::sync::broadcast::error::TryRecvError;

impl AnalyticsManager {
    pub fn new(mode: PrivacyMode) -> AnalyticsManager {
        AnalyticsManager {
            mode
        }
    }

    pub async fn spawn(&mut self, logger: LoggingManager, mut receiver: Receiver<ConnectionProtocolMessage>) {
        let (websend, webrec): (Sender<WebUIMessage>, Receiver<WebUIMessage>) = channel(20);

        let (wbtsend, wbtrec) = (websend.clone(), websend.subscribe());
        let l2 = logger.clone();
        tokio::spawn(async move {
            map_routes(l2, wbtsend, wbtrec).await;
        });

        match self.mode {
            PrivacyMode::None => {
                logger.debug_message(r#"Privacy setting is set to "None""#).await;
            }
            _ => {
                logger.debug_message(format!(r#"Privacy setting is set to "{:?}""#, self.mode)).await;
                let mut last = Instant::now();
                loop {
                    let inc_res = receiver.try_recv();

                    if let Ok(incoming) = inc_res {} else {
                        match inc_res.unwrap_err() {
                            TryRecvError::Empty => {}
                            TryRecvError::Closed => {}
                            TryRecvError::Lagged(e) => {}
                        }
                    }

                    logger.debug_message("This message is from the analytics manager").await;
                }
            }
        }
    }
}