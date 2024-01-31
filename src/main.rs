mod schema;
mod utils;

use utils::msfs::check_if_msfs_running;
use utils::simconnect::update_simconnect_config;

fn main() {
    let is_msfs_running = check_if_msfs_running();

    let update_config_result = update_simconnect_config();

    if let Err(error) = update_config_result {
        eprintln!("{}", error);
    }
}
