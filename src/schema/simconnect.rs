use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimConnectComm {
    #[serde(rename = "Descr")]
    pub description: String,
    #[serde(rename = "Protocol")]
    pub protocol: String,
    #[serde(rename = "Scope")]
    pub scope: String,
    #[serde(rename = "MaxClients")]
    pub max_clients: String,
    #[serde(rename = "MaxRecvSize")]
    pub max_recv_size: String,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port", skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "SimBase.Document")]
pub struct SimBaseDocument {
    #[serde(rename = "@Type")]
    pub document_type: String,
    #[serde(rename = "@version")]
    pub version: String,
    #[serde(rename = "Descr")]
    pub description: String,
    #[serde(rename = "Filename")]
    pub filename: String,
    #[serde(rename = "SimConnect.Comm", default)]
    pub simconnect_comm: Vec<SimConnectComm>,
}
