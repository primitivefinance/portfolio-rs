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

### Setup & Run

```bash
# Install submodule dependencies and generate rust bindings.
./setup.sh

# Run the app
cargo run
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