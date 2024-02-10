use local_ip_address::local_ip;
use std::{net::UdpSocket, sync::mpsc, thread, time::Duration};

pub static UDP_THREAD_STATUS_OK: &str = "udp-ok";
pub static UDP_THREAD_STATUS_ERROR: &str = "udp-error";

pub fn udp_broadcast_thread(sender: mpsc::Sender<String>, simconnect_port: String) {
    let local_ip_address = match local_ip() {
        Ok(ip_address) => ip_address,
        Err(_) => {
            sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
            return;
        }
    };

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

    let udp_data = format!("FSR_{}:{}", local_ip_address, simconnect_port);

    loop {
        match socket.send_to(udp_data.as_bytes(), "255.255.255.255:1234") {
            Ok(_) => sender.send(UDP_THREAD_STATUS_OK.to_string()).unwrap(),
            Err(_) => {
                sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
                break;
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
