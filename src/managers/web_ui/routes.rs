use crate::managers::logging::LoggingManager;

use tokio::sync::broadcast::{Sender, Receiver};
use crate::managers::web_ui::WebUIMessage;
use rocket::config::{Environment, LoggingLevel};
use rocket::http::ContentType;
use rocket::response::{Response, Body};
use std::io::Cursor;

#[get("/")]
fn root() -> &'static str {
    r#"
<html>
    <head>
        <title>Hello There</title>
    </head>
</html>"#
}

pub async fn map_routes(logger: LoggingManager, mut sender: Sender<WebUIMessage>, mut receiver: Receiver<WebUIMessage>) {
    // Specify custom configuration for Rocket
    let mut config = rocket::Config::build(Environment::Production);

    // set port, address, and log level (disable logs altogether because they do not use our logging manager and would wreck our formatting)
    config.port = 4894;
    config.address = "127.0.0.1".to_string();
    config.log_level = LoggingLevel::Off;

    // mount the config onto the server
    let mut range = rocket::custom(config.unwrap()).mount("/", routes![root]);

    logger.info("Web administration panel can be found at").await;
    logger.info("http://localhost:4880/").await;

    // start the webserver
    range.launch();
}