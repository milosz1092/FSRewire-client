use std::{net::UdpSocket, sync::mpsc, thread, time::Duration};

pub static UDP_THREAD_STATUS_OK: &str = "udp-ok";
pub static UDP_THREAD_STATUS_ERROR: &str = "udp-error";

pub fn udp_broadcast_thread(sender: mpsc::Sender<String>) {
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

    let broadcast_addr = "255.255.255.255:1234";
    let message = "Hello, world!";

    loop {
        match socket.send_to(message.as_bytes(), broadcast_addr) {
            Ok(_) => sender.send(UDP_THREAD_STATUS_OK.to_string()).unwrap(),
            Err(_) => {
                sender.send(UDP_THREAD_STATUS_ERROR.to_string()).unwrap();
                break;
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
