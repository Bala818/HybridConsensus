# HybridConsensus Project

## Overview
This project implements a simple blockchain balances module in Rust using `BTreeMap`. It supports:
- Adding users with balances.
- Retrieving user balances.
- Transferring balances between users safely.

## Features
- **State Management**: Uses `BTreeMap` for efficient balance tracking.
- **Safe Transfers**: Ensures secure balance transfers with overflow protection.
- **Unit Tests**: Comprehensive tests for all major functionalities.

## How to Run
1. Install Rust: [Rust Installation](https://www.rust-lang.org/tools/install)
2. Clone the repository:
   ```bash
   git clone <repository_url>
   cd HybridConsensus


3. Build and run the project

        cargo build
        cargo run

4. Run tests

        cargo test
