#!/bin/bash
echo "I hope you have rustup and cargo or I'm totally going to fail"
echo "Changing to rustup to nightly as required by Rocket"
cd server
rustup override set nightly
echo "If this run step fails, please run 'rustup update && cargo update' as reccomended at https://rocket.rs/v0.4/guide/getting-started/"
echo "This may take a while the first go..."
cargo run