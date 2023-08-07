#! /bin/bash

# Removes compiled contracts.
forge clean

# Installs the correct submodules.
forge install foundry-rs/forge-std --no-commit
forge install primitivefinance/portfolio@7247238a4da557ebae166c165ac6cd61ece65a68 --no-commit

# This compiles the contracts, be sure to use the correct compiler settings if using in production.
# Generates bindings to /out/bindings
forge bind --crate-name bindings --overwrite --via-ir --force

echo "Completed setup."