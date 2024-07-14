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

fn ask(system_prompt: &str, prompt: &str, model: &str) -> Result<String> {
  let client = Client::new();

  let api_key =
    std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

  let url = "https://api.openai.com/v1/chat/completions";

  let response = client
    .post(url)
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&json!({
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": prompt}
        ]
    }))
    .send()?;

  let json: serde_json::Value = response.json()?;

  Ok(
    json["choices"][0]["message"]["content"]
      .as_str()
      .unwrap()
      .to_string(),
  )
}

fn main() {
  dotenv().ok();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
