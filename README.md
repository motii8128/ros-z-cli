# hiroz-cli
[![Rust](https://github.com/motii8128/ros-z-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/ros-z-cli/actions/workflows/rust.yml)

Command line tool for creating a workspace and cargo package that use the colcon build system in hiroz.

# Usage
### install
```
git clone https://github.com/motii8128/hiroz-cli.git
cd hiroz-cli
cargo install --path .
```

### show help
```
hiroz -h
hiroz --help
```

### create workspace
create workspace and Cargo.toml
```
hiroz ws <WORKSPACE_NAME>
```

### create package
create cargo package with package.xml and ros-z depend
```
hiroz pkg <PACKAGE_NAME>
```