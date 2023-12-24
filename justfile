list:
  @just --list

help:
  @just list

check *FLAGS:
  cargo clippy --examples --all-targets --all-features --workspace {{FLAGS}}

test *FLAGS:
  cargo nextest run --all-features --workspace {{FLAGS}}

bench *FLAGS:
  cargo bench --all-targets --workspace {{FLAGS}}

run *FLAGS:
  RUST_LOG=trace RUST_BACKTRACE=full cargo run -- {{FLAGS}}

cov:
  cargo tarpaulin --engine llvm

setup:
  cargo install cargo-nextest
  cargo install cargo-tarpaulin

  pre-commit install --hook-type pre-commit --hook-type commit-msg
