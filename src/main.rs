mod schema;
mod utils;
use utils::msfs::check_if_msfs_running;
use utils::simconnect::update_simconnect_config;

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
