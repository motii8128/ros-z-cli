use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesPI, BytesStart, BytesText, Event};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

use std::process::Command;

pub fn pkg_action(pkg_name: String) {
    let src_path = Path::new("./src");

    if !src_path.exists() {
        crate::log_err!("\"src\" folder not found.");
        crate::log_err!("Please move to the workspace directory before use this command.");
        crate::log_err!("Shutdown ROS-Z-CLI");
        return;
    }

    crate::log_info!("create package using \"cargo new\" ...");
    let cargo_new_status = cargo_new(pkg_name.clone());
    if !cargo_new_status {
        return;
    }

    crate::log_info!("add \"package.xml\" to {}", pkg_name.clone());
    let pkg_xml_status = create_package_xml(pkg_name.clone());
    if !pkg_xml_status {
        return;
    }

    crate::log_info!("add depend to Cargo.toml");
    add_depend_to_cargo_toml(pkg_name.clone());

    crate::log_info!("complete task to create package. {}", pkg_name);
    crate::log_info!("End ROS-Z-CLI");
}

fn cargo_new(pkg_name: String) -> bool {
    let status = Command::new("cargo")
        .arg("new")
        .arg(pkg_name.clone())
        .current_dir("./src") // ← ここでsrcフォルダ内に移動
        .status();

    match status {
        Ok(status) => {
            if status.success() {
                crate::log_info!("Success to create package.");
                true
            } else {
                false
            }
        }
        Err(e) => {
            crate::log_err!("Failed to run \"cargo new {}\"", pkg_name.clone());
            crate::log_err!("{}", e);
            crate::log_err!("Aborting package creation");
            false
        }
    }
}

fn create_package_xml(pkg_name: String) -> bool {
    let file_path = format!("./src/{}/package.xml", pkg_name);
    let create_file_result = File::create(file_path);

    match create_file_result {
        Ok(xml_file) => {
            crate::log_info!("Success to create package.xml");
            let writer = BufWriter::new(xml_file);
            let mut xml_writer = Writer::new_with_indent(writer, b' ', 2);

            crate::log_info!("writing code to package.xml");

            let _ = xml_writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)));
            // model
            let _ = xml_writer.write_event(
                Event::PI(
                    BytesPI::new(
                        r#"xml-model href="http://download.ros.org/schema/package_format3.xsd" schematypens="http://www.w3.org/2001/XMLSchema""#
                    )
                )
            );

            // package
            let mut elem = BytesStart::new("package");
            elem.push_attribute(("format", "3"));
            let _ = xml_writer.write_event(Event::Start(elem));

            // name
            let _ = write_text(&mut xml_writer, "name", &pkg_name);

            // version
            let _ = write_text(&mut xml_writer, "version", "0.0.0");

            // description
            let _ = write_text(&mut xml_writer, "description", "");

            // maintainer
            crate::log_info!("Get git email and user name.");
            crate::log_info!("Email : {}", get_git_email());
            crate::log_info!("User Name : {}", get_git_name());
            let mut maintainer = BytesStart::new("maintainer");
            maintainer.push_attribute(("email", get_git_email().as_str()));
            let _ = xml_writer.write_event(Event::Start(maintainer));
            let _ = xml_writer.write_event(Event::Text(BytesText::new(get_git_name().as_str())));
            let _ = xml_writer.write_event(Event::End(BytesEnd::new("maintainer")));

            // license
            let _ = write_text(&mut xml_writer, "license", "");

            // test_depend
            let _ = write_text(&mut xml_writer, "test_depend", "ament_lint_auto");
            let _ = write_text(&mut xml_writer, "test_depend", "ament_lint_common");

            // export
            let _ = xml_writer.write_event(Event::Start(BytesStart::new("export")));
            // build_type
            let _ = write_text(&mut xml_writer, "build_type", "ament_cargo");
            let _ = xml_writer.write_event(Event::End(BytesEnd::new("export")));

            // end package;
            let _ = xml_writer.write_event(Event::End(BytesEnd::new("package")));

            crate::log_info!("Success to writing code to package.xml");
            true
        }
        Err(e) => {
            crate::log_err!("Failed to create package.xml");
            crate::log_err!("{}", e);
            false
        }
    }
}

fn add_depend_to_cargo_toml(pkg_name: String) {
    let toml = format!("./src/{}/Cargo.toml", pkg_name);
    let toml_path = Path::new(toml.as_str());

    let mut toml_file = OpenOptions::new()
        .append(true)
        .create(false)
        .open(toml_path)
        .unwrap();

    // ros-z = { git = "https://github.com/ZettaScaleLabs/ros-z.git" }
    // ros-z-msgs = { git = "https://github.com/ZettaScaleLabs/ros-z.git" }  # Standard ROS 2 message types
    // tokio = { version = "1", features = ["full"] }  # Async runtime
    let _ = writeln!(
        toml_file,
        "ros-z = {{ git = \"https://github.com/ZettaScaleLabs/ros-z.git\" }}"
    );
    let _ = writeln!(
        toml_file,
        "ros-z-msgs = {{ git = \"https://github.com/ZettaScaleLabs/ros-z.git\" }}  # Standard ROS 2 message types"
    );
    let _ = writeln!(
        toml_file,
        "tokio = {{ version = \"1\", features = [\"full\"] }}  # Async runtime"
    );
}

fn get_git_email() -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .output()
        .expect("Failed to execute git config.");

    let email = String::from_utf8_lossy(&output.stdout).trim().to_string();
    email
}

fn get_git_name() -> String {
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("Failed to execute git config.");

    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    name
}

fn write_text<W: std::io::Write>(
    writer: &mut Writer<W>,
    tag: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    writer.write_event(Event::Start(BytesStart::new(tag)))?;
    writer.write_event(Event::Text(BytesText::new(text)))?;
    writer.write_event(Event::End(BytesEnd::new(tag)))?;
    Ok(())
}
