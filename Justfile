default:
  just --list

run day:
  cargo run --release {{day}}
alias day := run

run-day10:
    RUST_MIN_STACK=104857600 cargo run --release 10

run-all:
    cargo run --release 0

flamegraph day:
  PERF=/usr/lib/linux-tools/5.15.0-89-generic/perf cargo flamegraph --profile flamegraph --output flamegraph_day_{{day}}.svg -- {{day}}

benchmark day:
  cargo build --release
  hyperfine --warmup 50 'target/release/aoc_2023 {{day}}'

lint:
  cargo clippy

tests:
  RUST_MIN_STACK=104857600 cargo test --color=always