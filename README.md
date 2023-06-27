# snake-raylib
A simple implementation of the classic arcade game Snake using [raylib-rs](https://github.com/deltaphc/raylib-rs).


## Controls
`W`, `A`, `S`, `D` to change direction. Left-click to restart game.

## Building
This project was tested using Windows 10 and Rust 1.70.0, and may not build properly with other systems and/or versions of Rust.
To build from source, make sure you have a working [Cargo](https://doc.rust-lang.org/cargo/) installation.
```bash
git clone https://github.com/jkmuriithi/snake-raylib.git
cd snake-raylib
cargo run

# Create a game executable in ./target/release
cargo build --release
```

## Todo
- Configuration file system for in-game constants
- Linux/MacOS build tests

## Contributing
Though this was a personal project I made for the sake of learning Rust and 2D graphics programming, I'm open to any and all
PRs which improve it. Before making a PR, please make sure that the crate builds with the latest stable version of Rust, and that
your code has been properly formatted using `cargo fmt`.


