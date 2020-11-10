#!/usr/bin/env bash

# Set required environment variables
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

# Remove artifacts from previous run
rm -f target/debug/deps/*.gcda

# Run tests
cargo test -v

# Generate HTML report

if [ -f "./grcov" ]; then
    ./grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
else
    grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
fi
