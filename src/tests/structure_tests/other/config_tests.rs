use crate::managers::implementors::managers::config::BASE_CONFIG;
use crate::managers::config::pre::PreConfig;

use toml::de::Error;

#[test]
fn test_empty_config_parse() {
    let manager: Result<PreConfig, Error> = toml::from_str("");

    if manager.is_ok() {
        dbg!(manager.unwrap().convert());
    } else {
        dbg!(manager.unwrap_err());
    }
}

#[test]
fn test_config_parse() {
    let manager: Result<PreConfig, Error> = toml::from_str(BASE_CONFIG);

    if manager.is_ok() {
        dbg!(manager.unwrap().convert());
    } else {
        dbg!(manager.unwrap_err());
    }
}