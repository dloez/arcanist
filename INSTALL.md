# Installation guide
Use one of the following methods

## Automatic installation (recommended)
Supported OS:
- Linux
- macOS

Requisites:
- wget

1. Run `curl https://raw.githubusercontent.com/dloez/arcanist/main/install.sh | sh`.

## Manual install
Use this method if your OS is currently not available in the [automatic installation](https://github.com/dloez/arcanist/blob/main/INSTALL.md#automatic-installation).

1. Download your platform binary from the [release page](https://github.com/dloez/arcanist/releases/tag/v0.1.0).
2. Place the binary in a directory that is on the system `PATH` or add the directory to the `PATH`.
3. Give execution permission on Unix system (`chmod +x arcanist`).

## Installing from source
Use this method if you cannot find a pre-built binary for your platform. Arcanist does not use any platform specific code, so if your platform is listed in the [supported Rust platforms](https://doc.rust-lang.org/nightly/rustc/platform-support.html), you should be able to easily build Arcanist.

1. Check if your platform is [supported by Rust](https://doc.rust-lang.org/nightly/rustc/platform-support.html).
2. [Download and install the Rust toolchain](https://www.rust-lang.org/tools/install) version >= 1.70.0. The installer will configure the required targets for your platform automatically, but depending on your platform you may need additional steps.
3. Clone the repository.
4. Run `cargo install`.
