# ros-z-cli
[![Rust](https://github.com/motii8128/ros-z-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/ros-z-cli/actions/workflows/rust.yml)

Command line tool for creating a workspace and cargo package that use the colcon build system in ros-z.

# Usage
### install
```
git clone https://github.com/motii8128/ros-z-cli.git
cd ros-z-cli
cargo install --path .
```

### show help
```
rosz -h
rosz --help
```

### create workspace
create workspace and Cargo.toml
```
rosz ws <WORKSPACE_NAME>
```

### create package
create cargo package with package.xml and ros-z depend
```
rosz pkg <PACKAGE_NAME>
```