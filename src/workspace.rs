use std::{fs, io::Write, path::Path};

pub fn workspace_action(name: String) {
    let ws_and_src = format!("./{}/src", name);
    let ws_and_src_path = Path::new(ws_and_src.as_str());

    crate::log_info!("start to create workspace. name : {}", name.clone());
    if ws_and_src_path.exists() {
        crate::log_warn!("workspace {} is already exists.", name);
        crate::log_err!("Shutdown ROS-Z-CLI...");
        return;
    }

    let _ = fs::create_dir_all(ws_and_src_path);
    crate::log_info!("success to create workspace");

    let cargo_toml = format!("{}/Cargo.toml", ws_and_src);
    let cargo_toml_path = Path::new(cargo_toml.as_str());

    crate::log_info!("start to create Cargo.toml");

    match fs::File::create(cargo_toml_path) {
        Ok(mut file) => {
            crate::log_info!("Success to create {}", cargo_toml.clone());
            let _ = file.write_all("[workspace]\nresolver = \"3\"\nmembers=[]".as_bytes());
        }
        Err(_e) => {
            crate::log_err!("Failed to create {}", cargo_toml.clone());
            crate::log_err!("Shutdown ROS-Z-CLI...");
            return;
        }
    }

    crate::log_info!("complete task to create workspace. {}", name);
    crate::log_info!("End ROS-Z-CLI");
}
