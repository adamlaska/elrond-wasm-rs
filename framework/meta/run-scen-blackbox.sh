#!/bin/bash

# Run scen-blackbox for digital-cash
echo "Running scen-blackbox for ping-pong-egld..."
cargo run scen-blackbox --overwrite --path ../../contracts/examples/ping-pong-egld

echo "Done!"
