use {
  crate::{
    arguments::Arguments, model::OpenAIModel, options::Options,
    subcommand::Subcommand,
  },
  anyhow::{anyhow, Result},
  clap::Parser,
  dialoguer::{theme::ColorfulTheme, Confirm},
  dotenv::dotenv,
  include_dir::{include_dir, Dir},
  regex::Regex,
  reqwest::blocking::Client,
  serde_json::json,
  similar::{ChangeTag, TextDiff},
  std::{
    fs,
    path::{Path, PathBuf},
    process,
    str::FromStr,
  },
};

static PROMPT_DIR: Dir = include_dir!("prompts");

mod arguments;
mod model;
mod options;
mod subcommand;

fn main() {
  dotenv().ok();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
