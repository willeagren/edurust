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

## Building for Release
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in *target/release*.