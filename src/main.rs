use dirs;

#[macro_use]
extern crate serde_derive;
extern crate encoding;

use std::fs;

use encoding::{DecoderTrap, EncoderTrap};

use quick_xml::events::Event;
use quick_xml::reader::Reader;

static SERVER_ADDR: &str = "0.0.0.0";
static SERVER_PORT: &str = "500";

fn check_if_msfs_running() -> bool {
    /* Opened SimConnect pipe indicates that MSFS2020 is running */
    fs::metadata("\\\\.\\pipe\\Microsoft Flight Simulator\\SimConnect").is_ok()
}

fn read_windows1252_file(file_path: &str) -> Result<String, String> {
    let content =
        fs::read(file_path).map_err(|e| format!("Error reading SimConnect.xml: {}", e))?;
    let encoding = encoding::label::encoding_from_whatwg_label("windows-1252").unwrap();
    let decoded = encoding.decode(&content, DecoderTrap::Replace);
    decoded.map_err(|e| format!("Error decoding Windows-1252 content: {}", e))
}

fn write_windows1252_file(file_path: &str, content: &str) -> Result<(), String> {
    let encoding = encoding::label::encoding_from_whatwg_label("windows-1252").unwrap();
    let encoded_content_result = encoding.encode(content, EncoderTrap::Replace);

    match encoded_content_result {
        Ok(encoded_content) => fs::write(file_path, encoded_content)
            .map_err(|e| format!("Error writing SimConnect.xml: {}", e)),
        Err(err) => Err(format!("Error encoding content to Windows-1252: {}", err)),
    }
}

fn get_simconnect_xml_path() -> String {
    if let Some(user_home) = dirs::home_dir() {
        let sim_connect_file_name = "SimConnect.xml";

        let user_home_str = user_home
            .to_str()
            .expect("Failed to convert path to string");

        let xml_file_path_1 = format!(
            "{}\\AppData\\Local\\Packages\\Microsoft.FlightSimulator_8wekyb3d8bbwe\\LocalCache\\{}",
            user_home_str, sim_connect_file_name
        );
        let xml_file_path_2 = format!(
            "{}\\AppData\\Roaming\\Microsoft Flight Simulator\\{}",
            user_home_str, sim_connect_file_name
        );

        let path_1_exists = std::path::Path::new(&xml_file_path_1).exists();
        let path_2_exists = std::path::Path::new(&xml_file_path_2).exists();

        if path_1_exists {
            xml_file_path_1
        } else if path_2_exists {
            xml_file_path_2
        } else {
            panic!("Unable to determine SimConnect XML path.");
        }
    } else {
        panic!("Unable to determine user home directory.");
    }
}

fn update_simconnect_config() -> Result<(String, String), String> {
    let xml_file_path = get_simconnect_xml_path();

    let xml_content = read_windows1252_file(&xml_file_path)?;
    // let xml_content = xml_content.replace("Windows-1252", "UTF-8");

    let mut reader = Reader::from_str(&xml_content);

    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                let path_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let last_path = paths.last_mut();

                match last_path {
                    Some(path) => path.push(path_name),
                    None => {
                        let mut first_path = Vec::new();
                        first_path.push(path_name);
                        paths.push(first_path);
                    }
                }
            }

            Ok(Event::End(_)) => {
                println!("paths size {}", paths.len());
                let last_path = paths.last_mut();

                if let Some(path) = last_path {
                    if !path.is_empty() {
                        path.pop();
                    } else {
                        paths.push(Vec::new());
                    }
                }
            }

            _ => (),
        }

        buf.clear();
    }

    // Print paths
    println!("Formatted Path: {:#?}", paths);

    let xml_new_content = xml_content.replace("UTF-8", "Windows-1252");

    write_windows1252_file(&xml_file_path, &xml_new_content)?;

    Ok(("none".to_string(), "none".to_string()))
}

fn main() {
    let is_msfs_running = check_if_msfs_running();

    if is_msfs_running {
        println!("MSFS is running");
    } else {
        println!("MSFS not running");
    }

    match update_simconnect_config() {
        Ok((address, port)) => println!("SimConnect Config - Address: {}, Port: {}", address, port),
        Err(err) => eprintln!("Error updating SimConnect config: {}", err),
    }
}
