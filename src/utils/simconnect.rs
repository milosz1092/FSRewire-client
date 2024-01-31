extern crate quick_xml;
use dirs;
use serde::Serialize;

use quick_xml::de::from_str as xml_from_string;
use quick_xml::se::Serializer as XmlSerializer;

use super::file::{read_windows1252_file, write_windows1252_file};

use crate::schema::simconnect::{SimBaseDocument, SimConnectComm};

static SERVER_ADDR: &str = "0.0.0.0";
static SERVER_PORT: &str = "500";

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

pub fn update_simconnect_config() -> Result<(String, String), String> {
    let xml_file_path = get_simconnect_xml_path();

    let xml_content = read_windows1252_file(&xml_file_path)?;

    let mut config: SimBaseDocument = xml_from_string(&xml_content)
        .map_err(|e| format!("Error parsing SimConnect.xml: {}", e))?;

    let mut ipv4_address = Some(SERVER_ADDR.to_string());
    let mut ipv4_port = Some(SERVER_PORT.to_string());
    let mut ipv4_found = false;

    for comm_section in &mut config.simconnect_comm {
        if comm_section.protocol == "IPv4" && !comm_section.description.contains("Dynamic") {
            ipv4_address = match &comm_section.address {
                Some(_) => Some(SERVER_ADDR.to_string()),
                None => Some(SERVER_ADDR.to_string()),
            };
            ipv4_port = match &comm_section.port {
                Some(port) => Some(port.clone()),
                None => Some(SERVER_PORT.to_string()),
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
