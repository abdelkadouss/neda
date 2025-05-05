set shell := ["nu", "-c"]

alias r := run
alias b := build
alias t := test
alias w := watch

_default:
  @just --list

# run bins (cli, lib)
run bin *args:
  #!/usr/bin/env nu
  if "{{bin}}" == "cli" {
    cargo run --bin neda {{args}}
  } else {
    cargo run --bin {{bin}} {{args}}
  }

# build app crates
build:
  cargo build --workspace --release

# test crates (available options: --watch)
test *opt:
  #!/usr/bin/env nu
  if "{{opt}}" == "--watch" {
    watchexec -w lib -w cli -w Cargo.toml -r nu ./scripts/run_tests_and_examples.nu;
  } else {
    nu ./scripts/run_tests_and_examples.nu;
  }

# watch crates the files changes and run the crate (default: neda-cli)
watch crate = "neda" *args:
  #!/usr/bin/env nu
  if "{{crate}}" == "cli" {
    watchexec -w cli -w Cargo.toml -r just run neda {{args}}
  } else {
    watchexec -w cli -w lib -w Cargo.toml -r just run {{crate}} {{args}}
  }
