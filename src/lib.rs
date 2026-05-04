mod pkg;
mod workspace;
mod help;

use std::env;

pub fn run()
{
    let args = env::args();

    let mut args_vec = Vec::new();

    for arg in args {
        args_vec.push(arg);
    }

    if args_vec.len() < 2 {
        log_warn!("Missing required arguments.");
        log_warn!(
            "Use '-h' or '--help' to display the help message and view detailed usage instructions."
        );
        return;
    }

    match args_vec[1].as_str() {
        "ws" => {
            let ws_name = args_vec[2].clone();

            workspace::workspace_action(ws_name);
        }
        "pkg" => {
            let pkg_name = args_vec[2].clone();

            pkg::pkg_action(pkg_name);
        }
        "-h" => help::help(),
        "--help" => help::help(),
        _ => {
            log_err!("Invalid arguments.");
            log_err!(
                "Use '-h' or '--help' to display the help message and view detailed usage instructions."
            );
        }
    }
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
