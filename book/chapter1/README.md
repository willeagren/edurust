# Installation, Hello World!, Hello Cargo!
The first step is to install Rust. We’ll download Rust through `rustup`, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.

## Installing `rustup` on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:

`$ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`

The command downloads a script and starts the installation of the `rustup` tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

`Rust is installed now. Great!`

## Updating and Uninstalling
Once Rust is installed via `rustup`, when a new version of Rust is released, updating to the latest version is easy. From your shell, run the following update script:
`$ rustup update`
To uninstall Rust and rustup, run the following uninstall script from your shell:
`$ rustup self uninstall`

