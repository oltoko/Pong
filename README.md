# Pong

Pong is a simple implementation of the classic Pong game using the [ggez](https://github.com/ggez/ggez) Rust game engine. This project aims to demonstrate basic game development concepts like rendering, input handling, and game logic updates.

## Requirements

To build and run the project, you need to have [Rust](https://www.rust-lang.org/tools/install) and its package manager, Cargo, installed on your system.

## Building and Running

To build the project, open a terminal in the project directory and run the following command:

```sh
cargo build --release
```

This will create an executable in the `target/release` directory. To run the game, execute the following command:

```sh
cargo run --release
```

## Controls

- Player 1 (left paddle):
  - `W`: Move up
  - `S`: Move down
- Player 2 (right paddle):
  - `Up Arrow`: Move up
  - `Down Arrow`: Move down

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

