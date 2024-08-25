use {
  crate::{
    arguments::Arguments, config::Config, model::Model, options::Options,
    provider::Provider, subcommand::Subcommand,
  },
  anyhow::{anyhow, ensure},
  clap::Parser,
  dialoguer::{theme::ColorfulTheme, Confirm},
  dotenv::dotenv,
  include_dir::{include_dir, Dir},
  regex::Regex,
  reqwest::blocking::Client,
  serde::{Deserialize, Serialize},
  serde_json::json,
  similar::{ChangeTag, TextDiff},
  std::{
    fs,
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
  xdg::BaseDirectories,
};

static PROMPT_DIR: Dir = include_dir!("prompts");

mod arguments;
mod config;
mod model;
mod options;
mod provider;
mod subcommand;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  dotenv().ok();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
