build-win:
    cargo build --release
    mv ./target/release/sudoku.exe ./target/release/sudoku-x86_64-pc-windows-msvc.exe

build-mac:
    cargo build --release
    mv ./target/release/sudoku ./target/release/sudoku-x86_64-apple-darwin

build-linux:
    cargo build --release
    mv ./target/release/sudoku ./target/release/sudoku-x86_64-unknown-linux-gnu