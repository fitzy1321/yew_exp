default:
    trunk serve

build:
    trunk clean
    trunk build

release:
    rm -rf release
    trunk clean
    trunk build -M --release --dist release

test:
    cargo test --target wasm32-unknown-unknown
