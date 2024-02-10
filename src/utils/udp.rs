use std::{net::UdpSocket, sync::mpsc, thread, time::Duration};

pub static UDP_THREAD_STATUS_OK: &str = "udp-ok";
pub static UDP_THREAD_STATUS_ERROR: &str = "udp-error";

static UDP_PACKET_PREFIX: &str = "FSR_SMC";
static UDP_BROADCAST_ADDRESS: &str = "255.255.255.255:1234";

pub fn udp_broadcast_thread(sender: mpsc::Sender<String>, simconnect_port: String) {
    let mut is_success_sent = false;

    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(socket) => socket,
        Err(_) => {
            sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
            return;
        }
    };

    if let Err(_) = socket.set_broadcast(true) {
        sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
        return;
    }

    let udp_data = format!("{}:{}", UDP_PACKET_PREFIX, simconnect_port);

    loop {
        match socket.send_to(udp_data.as_bytes(), UDP_BROADCAST_ADDRESS) {
            Ok(_) => {
                if !is_success_sent {
                    sender.send(UDP_THREAD_STATUS_OK.to_string()).unwrap();
                    is_success_sent = true;
                }
            }
            Err(_) => {
                sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
                break;
            }
        }

        thread::sleep(Duration::from_secs(10));
    }
}
