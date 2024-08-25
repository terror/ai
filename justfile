set dotenv-load

export EDITOR := 'nvim'

alias b := build
alias f := fmt
alias r := run

default:
  just --list

build:
  cargo build

fmt:
  cargo fmt

forbid:
  ./bin/forbid

install:
  cargo install --path .

run *args:
  cargo run -- {{args}}

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
