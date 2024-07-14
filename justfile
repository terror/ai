set dotenv-load

export EDITOR := 'nvim'

default:
  just --list

fmt:
  cargo +nightly fmt
