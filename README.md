# portfolio-rs

Minimalist toolkit for building rust applications on top of the [portfolio](https://github.com/primitivefinance/portfolio) protocol.

## Installation

#### [Required] Foundry. [Source](https://github.com/foundry-rs/foundry).
If not installed, run the following:
```bash
# First install foundryup
curl -L https://foundry.paradigm.xyz | bash

# Restart terminal or reload `PATH`, then run foundryup
foundryup
```

### Setup

```bash
# Install submodule dependencies and generate rust bindings.
./setup.sh

# Update .env with RPC url and private key
cp .env.example .env
```

### Usage

```bash
# Run the app
cargo run -- --help

# [Optional] Install locally (breaks terminal output)
cargo install --path . --force
```

### Recompile

If you choose a different portfolio version, or install new dependencies, make sure to recompile and bind the contracts.

```bash
# Recompile rust bindings
./compile.sh
```

### Commands


*List*

Lists the available portfolio pools, including their pool id and TVL.

```bash
cargo run -- list
```

- `list` - Lists all the pools, including pool id, tokens, and estimated TVL if available.
- `info` - Prints a pool's state and configuration, if any.
- `action` - Performs an action on a pool, such as swap, add liquidity, remove liquidity, etc. [Required] Settings in portfolio.toml.