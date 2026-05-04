pub mod pkg;
pub mod workspace;

pub fn help() {
    log_info!("Usage:");

    log_info!("\t\tros-z [Options] [NAME](Enter your workspace or package name.)");

    log_info!("Options:");
    log_info!("\t\t-h, --help\tShow this help message.");

    log_info!("\t\tws\t\tcreate workspace and Cargo.toml");
    log_info!("\t\tpkg\t\tcreate package");
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        println!("{} {}", colored::Colorize::green(format!("[ROS-Z-CLI][INFO]").as_str()), colored::Colorize::green(format!($($arg)*).as_str()));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("{} {}", colored::Colorize::yellow(format!("[ROS-Z-CLI][WARN]").as_str()), colored::Colorize::yellow(format!($($arg)*).as_str()));
    };
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        println!("{} {}", colored::Colorize::red(format!("[ROS-Z-CLI][ERROR]").as_str()), colored::Colorize::red(format!($($arg)*).as_str()));
    };
}
