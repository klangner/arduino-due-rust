# blinking led example written in Rust for Arduino Due

This project is based on:

  * https://github.com/japaric/discovery
  * http://hannobraun.de/embedded/


## Prepare rust environment 

```sh
rustup default nightly
rustup component add rust-src
rustup toolchain install nightly
cargo install xargo
```

## Compile & Run

```sh
xargo build
./upload.sh
```
