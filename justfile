default:
    trunk serve --open

build:
    trunk build --dist build

release:
    trunk build -M --release --dist release
