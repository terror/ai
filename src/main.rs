use {
  crate::{
    arguments::Arguments, model::OpenAIModel, options::Options,
    subcommand::Subcommand,
  },
  anyhow::{anyhow, Result},
  clap::Parser,
  dialoguer::{theme::ColorfulTheme, Confirm},
  dotenv::dotenv,
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
