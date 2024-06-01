cargo build -p station_service --release
cargo build -p station_service --target x86_64-pc-windows-gnu --release
cargo build -p station_service --target aarch64-unknown-linux-musl --release