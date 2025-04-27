set shell := ["nu", "-c"]

alias r := run
alias b := build
alias t := test
alias w := watch

_default:
  @just --list

# run bins (cli, lib)
run bin:
  #!/usr/bin/env nu
  if "{{bin}}" == "cli" {
    cargo run --bin neda-cli
  } else {
    cargo run --bin {{bin}}
  }

# build app crates
build:
  cargo build --workspace --release

# test crates (available options: --watch)
test *opt:
  #!/usr/bin/env nu
  if "{{opt}}" == "--watch" {
    watchexec -w lib -w cli -w Cargo.toml nu ./scripts/run_tests_and_examples.nu;
  } else {
    nu ./scripts/run_tests_and_examples.nu;
  }

# watch crates the files changes and run the crate (default: neda-cli)
watch crate = "neda-cli":
  #!/usr/bin/env nu
  if "{{crate}}" == "cli" {
    watchexec -w cli -w Cargo.toml just run neda-cli
  } else {
    watchexec -w cli -w lib -w Cargo.toml just run {{crate}}
  }
