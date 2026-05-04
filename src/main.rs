use std::env;

fn main() {
    let args = env::args();

    let mut args_vec = Vec::new();

    for arg in args {
        args_vec.push(arg);
    }

    if args_vec.len() < 2 {
        ros_z_cli::log_warn!("Missing required arguments.");
        ros_z_cli::log_warn!(
            "Use '-h' or '--help' to display the help message and view detailed usage instructions."
        );
        return;
    }

    match args_vec[1].as_str() {
        "ws" => {
            let ws_name = args_vec[2].clone();

            ros_z_cli::workspace::workspace_action(ws_name);
        }
        "pkg" => {
            let pkg_name = args_vec[2].clone();

            ros_z_cli::pkg::pkg_action(pkg_name);
        }
        "-h" => ros_z_cli::help(),
        "--help" => ros_z_cli::help(),
        _ => {
            ros_z_cli::log_err!("Invalid arguments.");
            ros_z_cli::log_err!(
                "Use '-h' or '--help' to display the help message and view detailed usage instructions."
            );
        }
    }
}
