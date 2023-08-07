// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

// These imports will be caught by running `forge bind` in this codebase.
// Without these, we can't generate the bindings for the submodule dependency contracts.
import "portfolio/contracts/Portfolio.sol";
import "portfolio/contracts/strategies/NormalStrategy.sol";

/// @dev Don't need this conract. Ignore.
contract Ignore {}
