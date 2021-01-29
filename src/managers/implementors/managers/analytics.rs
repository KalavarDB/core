use crate::managers::analytics::{AnalyticsManager, InnerMemoryInfo, Analytics, MemoryInfo, DiskInfo, OSInfo, CPUInfo};
use crate::managers::config::post::PrivacyMode;
use crate::managers::logging::LoggingManager;
// use crate::managers::web_ui::routes::map_routes;
// use crate::managers::web_ui::WebUIMessage;
use crate::core_structures::connection_protocol::ConnectionProtocolMessage;
use crate::core::utils::connection_handling::api::opcode_parser::OpCodes;

use heim::cpu::{physical_count, logical_count, frequency};
use heim::memory::{Memory, Swap};

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
        // let (websend, webrec): (Sender<WebUIMessage>, Receiver<WebUIMessage>) = channel(20);
        //
        // let (wbtsend, wbtrec) = (websend.clone(), websend.subscribe());
        // let l2 = logger.clone();
        // tokio::spawn(async move {
        //     map_routes(l2, wbtsend, wbtrec).await;
        // });

        match self.mode {
            PrivacyMode::None => {
                logger.debug_message(r#"Privacy setting is set to "None""#).await;
            }
            _ => {
                logger.debug_message(format!(r#"Privacy setting is set to "{:?}""#, self.mode)).await;
                let mut last = Instant::now();
                let mut mem = InnerMemoryInfo {
                    total: 0.0,
                    used: 0.0,
                    free: 0.0,
                };

                loop {
                    let inc_res = receiver.try_recv();

                    if let Ok(incoming) = inc_res {
                        match incoming.opcode {
                            OpCodes::MemUse => {
                                let stats = incoming.inner.2.unwrap();

                                mem = InnerMemoryInfo {
                                    total: stats.1,
                                    used: stats.0,
                                    free: stats.1 - stats.0,
                                }
                            }
                            _ => {}
                        }
                    }
                    if last.elapsed().as_secs() > 60 * 1000 {
                        last = Instant::now();

                        let mut x = Analytics {
                            version: "".to_string(),
                            cpu: CPUInfo {
                                manufacturer: "".to_string(),
                                model: "".to_string(),
                                frequency: Default::default(),
                                logical_cores: 0,
                                physical_cores: 0,
                            },
                            operating_system: OSInfo {
                                os: "".to_string(),
                                version: "".to_string(),
                                release: "".to_string(),
                            },
                            disk_info: DiskInfo { partitions: 0, usage_percent: 0.0 },
                            mem_info: MemoryInfo {
                                physical: InnerMemoryInfo {
                                    total: 0.0,
                                    used: 0.0,
                                    free: 0.0,
                                },
                                swap: InnerMemoryInfo {
                                    total: 0.0,
                                    used: 0.0,
                                    free: 0.0,
                                },
                                kalavar_specific: InnerMemoryInfo {
                                    total: 0.0,
                                    used: 0.0,
                                    free: 0.0,
                                },
                            },
                        };

                        // let id = raw_cpuid::cpuid!();

                        logger.debug_message("This message is from the analytics manager").await;
                    }
                }
            }
        }
    }
}