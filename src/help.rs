use crate::log_info;

pub fn help() {
    log_info!("Usage:");

    log_info!("\t\thiroz [Options] [NAME](Enter your workspace or package name.)");

    log_info!("Options:");
    log_info!("\t\t-h, --help\tShow this help message.");

    log_info!("\t\tws\t\tcreate workspace and Cargo.toml");
    log_info!("\t\tpkg\t\tcreate package");
}