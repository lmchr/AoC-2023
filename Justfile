default:
  just --list

perf day:
  PERF=/usr/lib/linux-tools/5.15.0-89-generic/perf cargo flamegraph --profile flamegraph -- {{day}}

benchmark day:
  cargo build --release
  hyperfine --warmup 50 'target/release/aoc_2023 {{day}}'
lint:
  cargo clippy
