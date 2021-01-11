use std::fs::File;
use std::io::Read;

use reqwest::blocking::ClientBuilder;
use regex::Regex;
use semver::Version;
use std::str::FromStr;
use crates_io_api::{CrateResponse, Error};

// Define the line ending of the current system, if unix it is "\n" if windows it is "\r\n", this is important when reading and writing files
#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

mod reference;

fn main() {
    let mut query = ClientBuilder::new().user_agent("Kalavar Version Utility v1.0 <Thomas B. | tom.b.2k2@gmail.com>")
    let client = crates_io_api::SyncClient::new(
        "my_bot (help@my_bot.com)",
        std::time::Duration::from_millis(200),
    ).unwrap();
    let handle = File::open("./Cargo.toml");

    if handle.is_ok() {
        let (mut utd, mut ood, mut sav) = (0, 0, 0);
        let mut file = handle.unwrap();
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf);
        let content = String::from_utf8(buf).unwrap();

        let mut found_deps = false;
        for line in content.split(LINE_ENDING) {
            if line != "" {
                if line == "[dependencies]" {
                    found_deps = true;
                } else if found_deps == true {
                    if line.contains("{") {
                        let components: Vec<&str> = line.split(" = ").collect();
                        let container = components[0];

                        let pattern = Regex::new(r#"[\d.]+"#).unwrap();

                        let mut vcore = pattern.captures(line).unwrap().iter().last().unwrap().unwrap().as_str();
                        let mut pieces = vcore.split(".").collect::<Vec<&str>>();

                        if pieces.len() < 3 {
                            pieces.push("0");
                        }

                        let vfull: String = pieces.join(".");

                        let version = Version::from_str(vfull.as_str()).unwrap();

                        let crate_res: Result<CrateResponse, Error> = client.get_crate(container);
                        if crate_res.is_ok() {
                            let response = crate_res.unwrap();
                            let rcore = response.versions.first().unwrap().clone().num;
                            let mut rpieces = rcore.split(".").collect::<Vec<&str>>();

                            if rpieces.len() < 3 {
                                rpieces.push("0");
                            }
                            let rfull: String = pieces.join(".");
                            let remote = Version::from_str(rfull.as_str()).unwrap();

                            let mut color = "\x1b[0m";

                            if remote > version {
                                color = "\x1b[31m";
                                ood+=1
                            } else {
                                utd+=1;
                            }

                            println!("{} - {}{}\x1b[0m", container, color, version);
                        } else {
                            println!("{}* - \x1b[35m{}\x1b[0m", container, version);
                        }
                    } else {
                        let components: Vec<&str> = line.split(" = ").collect();
                        let container = components[0];

                        let vcore = components[1].split("\"").collect::<Vec<&str>>();
                        let mut pieces = vcore[1].split(".").collect::<Vec<&str>>();

                        if pieces.len() < 3 {
                            pieces.push("0");
                        }

                        let vfull: String = pieces.join(".");

                        let version = Version::from_str(vfull.as_str()).unwrap();
                        let crate_res: Result<CrateResponse, Error> = client.get_crate(container);
                        if crate_res.is_ok() {
                            let response = crate_res.unwrap();
                            let rcore = response.versions.first().unwrap().clone().num;
                            let mut rpieces = rcore.split(".").collect::<Vec<&str>>();

                            if rpieces.len() < 3 {
                                rpieces.push("0");
                            }
                            let rfull: String = pieces.join(".");
                            let remote = Version::from_str(rfull.as_str()).unwrap();

                            let mut color = "\x1b[0m";

                            if remote > version {
                                color = "\x1b[31m";
                                ood+=1
                            } else {
                                utd+=1;
                            }

                            println!("{} - {}{}\x1b[0m", container, color, version);
                        } else {
                            println!("{}* - \x1b[35m{}\x1b[0m", container, version);
                        }
                    }
                }
            }
        }
        println!("* Crate failed to return a version result");

        println!("Results: Up to date: {}   Updates available: {}   Security Advisories: {}", utd, ood, sav);
        println!("Total: {}", utd + ood);

    } else {
        println!("\x1b[41mFATAL ERROR\x1b[0m: Unable to locate Cargo.toml file")
    }
}
