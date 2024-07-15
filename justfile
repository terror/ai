set dotenv-load

export EDITOR := 'nvim'

alias b := build
alias f := fmt

default:
  just --list

build:
  cargo build

fmt:
  cargo +nightly fmt

forbid:
  ./bin/forbid
