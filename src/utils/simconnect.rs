extern crate quick_xml;
use dirs;
use serde::Serialize;

use quick_xml::de::from_str as xml_from_string;
use quick_xml::se::Serializer as XmlSerializer;

use super::file::{read_windows1252_file, write_windows1252_file};

use crate::schema::simconnect::{SimBaseDocument, SimConnectComm};

static SIMCONNECT_SERVER_ADDR: &str = "0.0.0.0";
static SIMCONNECT_SERVER_PORT: &str = "500";

fn get_simconnect_xml_path() -> String {
    if let Some(user_home) = dirs::home_dir() {
        let sim_connect_file_name = "SimConnect.xml";

        let user_home_str = user_home
            .to_str()
            .expect("Failed to convert path to string");

        let ms_store_filepath = format!(
            "{}\\AppData\\Local\\Packages\\Microsoft.FlightSimulator_8wekyb3d8bbwe\\LocalCache\\{}",
            user_home_str, sim_connect_file_name
        );
        let steam_edition_filepath = format!(
            "{}\\AppData\\Roaming\\Microsoft Flight Simulator\\{}",
            user_home_str, sim_connect_file_name
        );

        let ms_store_path_exists = std::path::Path::new(&ms_store_filepath).exists();
        let steam_version_path_exists = std::path::Path::new(&steam_edition_filepath).exists();

        if ms_store_path_exists {
            ms_store_filepath
        } else if steam_version_path_exists {
            steam_edition_filepath
        } else {
            panic!("Unable to determine SimConnect XML path.");
        }
    } else {
        panic!("Unable to determine user home directory.");
    }
}

pub fn update_simconnect_config() -> Result<(String, String), String> {
    let xml_file_path = get_simconnect_xml_path();

    let xml_content = read_windows1252_file(&xml_file_path)?;

    let mut config: SimBaseDocument = xml_from_string(&xml_content)
        .map_err(|e| format!("Error parsing SimConnect.xml: {}", e))?;

    let mut ipv4_address = Some(SIMCONNECT_SERVER_ADDR.to_string());
    let mut ipv4_port = Some(SIMCONNECT_SERVER_PORT.to_string());
    let mut ipv4_found = false;

    for comm_section in &mut config.simconnect_comm {
        if comm_section.protocol == "IPv4" && !comm_section.description.contains("Dynamic") {
            ipv4_address = match &comm_section.address {
                Some(_) => Some(SIMCONNECT_SERVER_ADDR.to_string()),
                None => Some(SIMCONNECT_SERVER_ADDR.to_string()),
            };
            ipv4_port = match &comm_section.port {
                Some(port) => Some(port.clone()),
                None => Some(SIMCONNECT_SERVER_PORT.to_string()),
            };

            comm_section.address = ipv4_address.clone();
            comm_section.port = ipv4_port.clone();

            ipv4_found = true;
            break;
        }
    }

    if !ipv4_found {
        config.simconnect_comm.push(SimConnectComm {
            protocol: "IPv4".to_string(),
            address: ipv4_address.clone(),
            port: ipv4_port.clone(),
            description: "Static IP4 port".to_string(),
            scope: "local".to_string(),
            max_clients: "64".to_string(),
            max_recv_size: "4188".to_string(),
        });
    }

    let mut output = String::new();

    output.push_str(r#"<?xml version="1.0" encoding="Windows-1252"?>"#);
    output.push_str("\n\n");

    let mut ser = XmlSerializer::new(&mut output);
    ser.indent(' ', 4);

    config.serialize(ser).unwrap();

    write_windows1252_file(&xml_file_path, &output)?;

    Ok((ipv4_address.unwrap(), ipv4_port.unwrap()))
}
