use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use reqwest::blocking::ClientBuilder;
use regex::Regex;
use semver::Version;
use crates_io_api::{CrateResponse, Error};
use std::process::exit;
use std::collections::HashMap;

// Define the line ending of the current system, if unix it is "\n" if windows it is "\r\n", this is important when reading and writing files
#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

// mod reference;

fn main() {
    let query = ClientBuilder::new().user_agent("Kalavar Version Utility v1.0 <Thomas B. | tom.b.2k2@gmail.com>").build().unwrap();

    let mut dirbytes: Vec<u8> = Vec::new();
    let _ = query.get("https://github.com/RustSec/advisory-db/archive/master.zip").send().unwrap().read_to_end(&mut dirbytes);

    let exe_dir = std::env::current_exe().unwrap().as_os_str().to_str().unwrap().to_string();

    let mut home_dir_vec = exe_dir.split("/").collect::<Vec<&str>>();
    home_dir_vec.pop();
    let path = format!("{}/security.zip", home_dir_vec.join("/"));

    let e = OpenOptions::new().write(true).read(true).create(true).open(path.as_str());

    if e.is_err() {
        match e.unwrap_err().kind() {
            _ => {
                println!("\x1b[41mFATAL ERROR\x1b[0m: Unable to update security advisory database");
                exit(1);
            }
        }
    }

    let mut db = e.unwrap();

    let write_status = db.write_all(dirbytes.as_slice());
    if write_status.is_ok() {
        let _ = db.flush();

        let repo = zip::read::ZipArchive::new(&mut db);

        if repo.is_ok() {
            let zipped = repo.unwrap();
            let base = "advisory-db-master/crates/";
            let mut advisories = HashMap::<&str, Vec<&str>>::new();
            for file in zipped.file_names() {
                if file.contains(base) {
                    let hostdir = file.split(base).collect::<Vec<&str>>();
                    let parts = hostdir[1].split("/").collect::<Vec<&str>>();

                    let crate_name = parts[0];
                    if parts.len() > 1 {
                        let file = parts[1];
                        if file != "" {
                            if advisories.contains_key(crate_name) {
                                advisories.get_mut(crate_name).unwrap().push(file);
                            } else {
                                advisories.insert(crate_name, vec![file]);
                            }
                        }
                    }
                }
                // advisories.
            }

            println!("\n");

            let client = crates_io_api::SyncClient::new(
                "my_bot (help@my_bot.com)",
                std::time::Duration::from_millis(100),
            ).unwrap();
            let handle = File::open("./Cargo.toml");

            if handle.is_ok() {
                let (mut utd, mut ood, mut sav) = (0, 0, 0);
                let mut file = handle.unwrap();
                let mut buf = Vec::<u8>::new();
                let _ = file.read_to_end(&mut buf);
                let content = String::from_utf8(buf).unwrap();
                println!("======================> Crate Version Checker 0.1 <======================");
                println!("Advisories  Crate                                        Version         ");
                println!("=========================================================================");
                let mut found_deps = false;
                for line in content.split(LINE_ENDING) {
                    if line != "" {
                        if line == "[dependencies]" {
                            found_deps = true;
                        } else if found_deps == true {
                            if line.contains("{") {
                                let components: Vec<&str> = line.split(" = ").collect();
                                let mut container = components[0].to_string();

                                let pattern = Regex::new(r#"[\d.]+"#).unwrap();

                                let vcore = pattern.captures(line).unwrap().iter().last().unwrap().unwrap().as_str();
                                let mut pieces = vcore.split(".").collect::<Vec<&str>>();

                                if pieces.len() < 3 {
                                    pieces.push("0");
                                }

                                let vfull: String = pieces.join(".");

                                let version = Version::parse(vfull.as_str()).unwrap();

                                let crate_res: Result<CrateResponse, Error> = client.get_crate(container.as_str());
                                if crate_res.is_ok() {
                                    let response = crate_res.unwrap();
                                    let mut rcore = "0.0.0".to_string();
                                    for ver in response.versions {
                                        let mut vnum = ver.num;

                                        let mut rpieces = vnum.split(".").collect::<Vec<&str>>();

                                        if rpieces.len() < 3 {
                                            rpieces.push("0");
                                        }
                                        vnum = rpieces.join(".");
                                        let parsed = Version::parse(vnum.as_str()).unwrap();
                                        let core = Version::parse(rcore.as_str()).unwrap();

                                        if parsed > core {
                                            rcore = vnum;
                                        }
                                    }
                                    let mut rpieces = rcore.split(".").collect::<Vec<&str>>();

                                    if rpieces.len() < 3 {
                                        rpieces.push("0");
                                    }
                                    let rfull: String = rpieces.join(".");
                                    let remote = Version::parse(rfull.as_str()).unwrap();

                                    let base_crate = container.clone();

                                    while container.len() < 44 {
                                        container = format!("{} ", container);
                                    }

                                    let mut color = "\x1b[32m";
                                    if remote > version {
                                        color = "\x1b[31m";
                                        if advisories.contains_key(base_crate.as_str()) {
                                            let adv_tot = advisories.get(base_crate.as_str()).unwrap().len();
                                            sav += adv_tot;
                                            let mut adv_count = format!("\x1b[41m{}\x1b[0m", adv_tot.to_string());
                                            while adv_count.len() < 19 {
                                                adv_count = format!(" {}", adv_count);
                                            }
                                            println!("{}  {} {}{}\x1b[0m -> \x1b[32m{}\x1b[0m", adv_count, container, color, version, remote);
                                        } else {
                                            println!("         \x1b[32m0\x1b[0m  {} {}{}\x1b[0m -> \x1b[32m{}\x1b[0m", container, color, version, remote);
                                        }
                                        ood += 1
                                    } else {
                                        if advisories.contains_key(base_crate.as_str()) {
                                            let adv_tot = advisories.get(base_crate.as_str()).unwrap().len();
                                            sav += adv_tot;
                                            let mut adv_count = format!("\x1b[41m{}\x1b[0m", adv_tot.to_string());
                                            while adv_count.len() < 19 {
                                                adv_count = format!(" {}", adv_count);
                                            }
                                            println!("{}  {} {}{}\x1b[0m", adv_count, container, color, version);
                                        } else {
                                            println!("         \x1b[32m0\x1b[0m  {} {}{}\x1b[0m", container, color, version);
                                        }
                                        utd += 1;
                                    }
                                } else {
                                    println!("{}* - \x1b[35m{}\x1b[0m", container, version);
                                }
                            } else {
                                let components: Vec<&str> = line.split(" = ").collect();
                                let mut container = components[0].to_string();

                                let vcore = components[1].split("\"").collect::<Vec<&str>>();
                                let mut pieces = vcore[1].split(".").collect::<Vec<&str>>();

                                if pieces.len() < 3 {
                                    pieces.push("0");
                                }

                                let vfull: String = pieces.join(".");

                                let version = Version::parse(vfull.as_str()).unwrap();
                                let crate_res: Result<CrateResponse, Error> = client.get_crate(container.as_str());
                                if crate_res.is_ok() {
                                    let response = crate_res.unwrap();
                                    let mut rcore = "0.0.0".to_string();
                                    for ver in response.versions {
                                        let mut vnum = ver.num;

                                        let mut rpieces = vnum.split(".").collect::<Vec<&str>>();

                                        if rpieces.len() < 3 {
                                            rpieces.push("0");
                                        }
                                        vnum = rpieces.join(".");
                                        let parsed = Version::parse(vnum.as_str()).unwrap();
                                        let core = Version::parse(rcore.as_str()).unwrap();

                                        if parsed > core {
                                            rcore = vnum;
                                        }
                                    }
                                    let mut rpieces = rcore.split(".").collect::<Vec<&str>>();

                                    if rpieces.len() < 3 {
                                        rpieces.push("0");
                                    }
                                    let rfull: String = rpieces.join(".");
                                    let remote = Version::parse(rfull.as_str()).unwrap();

                                    let base_crate = container.clone();

                                    while container.len() < 44 {
                                        container = format!("{} ", container);
                                    }

                                    let mut color = "\x1b[32m";
                                    if remote > version {
                                        color = "\x1b[31m";
                                        if advisories.contains_key(base_crate.as_str()) {
                                            let adv_tot = advisories.get(base_crate.as_str()).unwrap().len();
                                            sav += adv_tot;
                                            let mut adv_count = format!("\x1b[41m{}\x1b[0m", adv_tot.to_string());
                                            while adv_count.len() < 19 {
                                                adv_count = format!(" {}", adv_count);
                                            }
                                            println!("{}  {} {}{}\x1b[0m -> \x1b[32m{}\x1b[0m", adv_count, container, color, version, remote);
                                        } else {
                                            println!("         \x1b[32m0\x1b[0m  {} {}{}\x1b[0m -> \x1b[32m{}\x1b[0m", container, color, version, remote);
                                        }
                                        ood += 1
                                    } else {
                                        if advisories.contains_key(base_crate.as_str()) {
                                            let adv_tot = advisories.get(base_crate.as_str()).unwrap().len();
                                            sav += adv_tot;
                                            let mut adv_count = format!("\x1b[41m{}\x1b[0m", adv_tot.to_string());
                                            while adv_count.len() < 19 {
                                                adv_count = format!(" {}", adv_count);
                                            }
                                            println!("{}  {} {}{}\x1b[0m", adv_count, container, color, version);
                                        } else {
                                            println!("         \x1b[32m0\x1b[0m  {} {}{}\x1b[0m", container, color, version);
                                        }
                                        utd += 1;
                                    }
                                } else {
                                    println!("{}* - \x1b[35m{}\x1b[0m", container, version);
                                }
                            }
                        }
                    }
                }
                println!("=========================================================================");
                println!("Results: Up to date: {}   Updates available: {}   Security Advisories: {}", utd, ood, sav);
                println!("=========================================================================");
                println!("Total: {}", utd + ood);
                println!("=========================================================================");
                println!("* Crate failed to return a version result");
            } else {
                println!("\x1b[41mFATAL ERROR\x1b[0m: Unable to locate Cargo.toml file")
            }
        } else {
            println!("{:#?}", repo);
            println!("\x1b[41mFATAL ERROR\x1b[0m: Unable to read security advisory database");
            exit(1);
        }
    } else {
        println!("{}", write_status.unwrap_err());
        println!("\x1b[41mFATAL ERROR\x1b[0m: Unable to write to security advisory database");
    }
}
