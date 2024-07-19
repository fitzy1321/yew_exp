# Experiment Yew

What is it like, making a web app with [yew.rs](https://yew.rs)

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Just](https://github.com/casey/just) task runner  
    `brew install just`
3. Run the following to setup wasm dev environment on your local machine

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
```

## Run locally

Make sure everything above is done first ☝️.

To build and run server local: `just` or `just default`.

To generate a build without running the server: `just build`

And to build a release version: `just release`

All recipes are in `justfile`.

All these tasks (currently) use [Trunk](https://trunkrs.dev/) WASM Web Application Bundler.

## References

- <https://yew.rs/docs/getting-started/build-a-sample-app>
- <https://bulma.io/documentation>
- <https://trunkrs.dev/assets>
