#!/bin/bash

# Run scen-blackbox for adder
echo "Running scen-blackbox for adder..."
cargo run scen-blackbox  --overwrite --path ../../contracts/examples/adder

# Run scen-blackbox for digital-cash
echo "Running scen-blackbox for digital-cash..."
cargo run scen-blackbox --overwrite --path ../../contracts/examples/digital-cash

echo "Done!"
