default:
    trunk serve --open

build:
    trunk clean
    trunk build

release:
    rm -rf release
    trunk clean
    trunk build -M --release --dist release
