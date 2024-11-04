# Ramp Reimbursements + Transactions

This repo contains a script for processing transactions and reimbursements from Ramp and storing them in a SQL table for GP to find.

## Setup

1. Clone this repo
2. Install cargo.
3. Run `cargo build --release` to create an executable in `target/release/ramp`
4. Run `./target/release/ramp` to run the script

## Usage

This script requires a `config.toml` file in the root directory of the project. Look at the `example.toml` file for an example of what this file should look like.
