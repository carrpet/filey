# filey 
'filey' is a cool new tool that performs common operations with files.

## Building from source

### Prerequisites
This application was built with the Rust programming language.  Anyone wishing to build the source
code needs to have Rust and the cargo package manager installed: https://www.rust-lang.org/

### Testing
After cloning the repo, execute the following from the root directory of the project to run all the tests:
```
cargo test
```
Integration tests are located in the `tests` directory and unit tests are in the source code file.  Note that
all the the tests rely on an external filesystem.

### Building Binary
A binary can be built simply by executing 
```
cargo build -r
```
from the project root directory.  The binary will be placed in `<ROOT_PROJECT_DIR>/target/release`

### Building Debian Package
A Debian package can be built from the source code.  First ensure that the crate `cargo-deb` is installed:
```
cargo install cargo-deb
```

Then run 
```
cargo deb
```
in the project root directory.  The `.deb` package will most likely be placed in the `<ROOT_PROJECT_DIR/target/debian>` directory.

### Installation From Debian Package
Assuming you have a debian package install it with the command:
```
dpkg -i <PATH_TO_DEBIAN_PKG>/filey_<VERSION>_arm64.deb
```

Note that this command needs to be executed in superuser mode (sudo) since it is installed for all users in `/usr/bin`

## Usage 

Please see the CLI help documentation for up to date usage syntax.  Type `filey` to see all possible commands and `filey <COMMAND>`
to see detailed help documentation for each command. Note that currently all operations (except for del) are non-destructive.  They will not overwrite
existing files.
