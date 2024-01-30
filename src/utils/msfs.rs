use std::fs;

pub fn check_if_msfs_running() -> bool {
    /* Opened SimConnect pipe indicates that MSFS2020 is running */
    fs::metadata("\\\\.\\pipe\\Microsoft Flight Simulator\\SimConnect").is_ok()
}
