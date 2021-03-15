/sh
ip="192.168.1.6"
target="armv5te-unknown-linux-gnueabi"
bin_name=$(awk -F \" '/name/ {print $2}' Cargo.toml)
bin_path="target/${target}/release/${bin_name}"
cargo build --release --target $target
scp $bin_path robot@${ip}:programs/
