#! /bin/bash

# Clears bindings
forge clean

# This compiles the contracts, be sure to use the correct compiler settings if using in production.
# Generates bindings to /out/bindings
forge bind --crate-name bindings --overwrite --via-ir --force

echo "Completed compilation."