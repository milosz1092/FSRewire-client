use dirs;

#[macro_use]
extern crate serde_derive;
extern crate encoding;

use std::fs;

use encoding::{DecoderTrap, EncoderTrap};

#[derive(Debug, Deserialize, Serialize)]
struct SimConnectComm {
    #[serde(rename = "Descr")]
    description: String,
    #[serde(rename = "Protocol")]
    protocol: String,
    #[serde(rename = "Scope")]
    scope: String,
    #[serde(rename = "Port")]
    port: String,
    #[serde(rename = "MaxClients")]
    max_clients: String,
    #[serde(rename = "MaxRecvSize")]
    max_recv_size: String,
    #[serde(default, rename = "Address")]
    address: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SimBaseDocument {
    #[serde(rename = "Type")]
    document_type: String,
    #[serde(rename = "version")]
    version: String,
    #[serde(rename = "Descr")]
    description: String,
    #[serde(rename = "Filename")]
    filename: String,
    #[serde(rename = "SimConnect.Comm")]
    simconnect_comm: Vec<SimConnectComm>,
}

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
    let xml_content = xml_content.replace("Windows-1252", "UTF-8");

    let mut config: SimBaseDocument = serde_xml_rs::from_str(&xml_content)
        .map_err(|e| format!("Error parsing SimConnect.xml: {}", e))?;

    let mut ipv4_address = String::new();
    let mut ipv4_port = String::new();
    let mut ipv4_found = false;

    for comm_section in &mut config.simconnect_comm {
        if comm_section.protocol == "IPv4" && !comm_section.description.contains("Dynamic") {
            println!("IPv4 section found");
            ipv4_address = comm_section.address.clone();
            ipv4_port = comm_section.port.clone();
            ipv4_found = true;
            break;
        }
    }

    if ipv4_address.ne(SERVER_ADDR) {
        ipv4_address = SERVER_ADDR.to_string();
    }

    if ipv4_port.is_empty() {
        ipv4_port = SERVER_PORT.to_string();
    }

    if !ipv4_found {
        // Add the IPv4 section with the default values if it doesn't exist
        config.simconnect_comm.push(SimConnectComm {
            protocol: "IPv4".to_string(),
            address: ipv4_address.clone(),
            port: ipv4_port.clone(),
            description: "Static IP4 port".to_string(),
            scope: "local".to_string(),
            max_clients: "64".to_string(),
            max_recv_size: "4188".to_string(),
        });

        // Save the modified XML content back to the file
        let encoded_xml_content = serde_xml_rs::to_string(&config)
            .map_err(|e| format!("Error encoding SimBaseDocument to XML: {}", e))?;

        // write_windows1252_file(&xml_file_path, &encoded_xml_content)?;
    }

    Ok((ipv4_address, ipv4_port))
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
