set shell := ["nu", "-c"]

alias r := run
alias b := build
alias t := test
alias w := watch

default:
  @just --list

run:
  cargo run .

build:
  cargo build

test:
  cargo test

watch what = "run":
  watchexec -w src -w Cargo.toml just {{what}}
