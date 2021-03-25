#!/bin/sh

ip="192.168.1.6"
target="armv5te-unknown-linux-gnueabi"
bin_names=$(awk -F \" '/name/ {print $2}' Cargo.toml)
profile="debug"

if [ ${1:-none} = "release" ]; then 
	profile="release"
	cargo_flags="--release"
fi
cargo build $cargo_flags --target $target || exit 1

for bin_name in $bin_names; do
	bin_path="target/${target}/${profile}/${bin_name}"
	scp -C $bin_path robot@${ip}:programs/
done
