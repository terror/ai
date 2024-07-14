use {
  anyhow::{anyhow, Result},
  clap::Parser,
  dialoguer::{theme::ColorfulTheme, Confirm},
  regex::Regex,
  reqwest::blocking::Client,
  serde_json::json,
  similar::{ChangeTag, TextDiff},
  std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
  },
};

impl Arguments {
  fn run(self) -> Result<()> {
    match self.subcommand {
      Subcommand::Fill(fill) => fill.run(),
    }
  }
}

#[derive(Debug, Parser)]
struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand,
}

#[derive(Debug, Clone)]
enum OpenAIModel {
  GPT4,
  GPT4_32K,
  GPT3_5Turbo,
  GPT3_5turbo16k,
}

impl FromStr for OpenAIModel {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "gpt-4" => Ok(OpenAIModel::GPT4),
      "gpt-4-32k" => Ok(OpenAIModel::GPT4_32K),
      "gpt-3.5-turbo" => Ok(OpenAIModel::GPT3_5Turbo),
      "gpt-3.5-turbo-16k" => Ok(OpenAIModel::GPT3_5turbo16k),
      _ => Err(format!("Invalid model: {}", s)),
    }
  }
}

impl ToString for OpenAIModel {
  fn to_string(&self) -> String {
    match self {
      OpenAIModel::GPT4 => "gpt-4".to_string(),
      OpenAIModel::GPT4_32K => "gpt-4-32k".to_string(),
      OpenAIModel::GPT3_5Turbo => "gpt-3.5-turbo".to_string(),
      OpenAIModel::GPT3_5turbo16k => "gpt-3.5-turbo-16k".to_string(),
    }
  }
}

#[derive(Debug, Parser)]
struct Fill {
  #[clap(help = "Input file path")]
  file: String,
  #[clap(long, short, help = "Context file paths", num_args = 0..)]
  context: Vec<PathBuf>,
  #[clap(
    long,
    short,
    help = "Model name (default: 'gpt-3.5-turbo')",
    default_value = "gpt-3.5-turbo"
  )]
  model: OpenAIModel,
}

#[derive(Debug, Parser)]
enum Subcommand {
  Fill(Fill),
}

impl Fill {
  fn run(self) -> Result<()> {
    let code = fs::read_to_string(&self.file)?;

    let mut context_code = String::new();

    for context_file in &self.context {
      let content = fs::read_to_string(context_file)?;
      context_code.push_str(&content);
      context_code.push('\n');
    }

    let combined_code = format!("{}\n{}", context_code, code);
    let combined_code = import_context_files(&combined_code, &self.file)?;

    let tokens = token_count(&combined_code);
    let holes = find_holes(&combined_code);

    println!("Analysis complete:");
    println!("  - Holes found: {:?}", holes);
    println!("  - Total token count: {}", tokens);
    println!("  - Using model: {}", self.model.to_string());

    let mut updated_file_code = code.clone();

    if holes == vec!["??"] {
      println!("\nProcessing single '??' hole...");
      updated_file_code = self.process_hole(
        "??",
        &combined_code,
        &code,
        self.model.to_string(),
      )?;
    } else {
      println!("\nProcessing multiple holes...");
      for (index, hole) in holes.iter().enumerate() {
        println!(
          "\nProcessing hole {} of {}: '{}'",
          index + 1,
          holes.len(),
          hole
        );
        updated_file_code = self.process_hole(
          hole,
          &combined_code,
          &updated_file_code,
          self.model.to_string(),
        )?;
      }
    }

    fs::write(&self.file, &updated_file_code)?;
    println!("\nFile successfully updated: {}", self.file);

    Ok(())
  }

  fn process_hole(
    &self,
    hole: &str,
    combined_code: &str,
    current_code: &str,
    model: String,
  ) -> Result<String> {
    println!("Generating completion for hole: '{}'...", hole);

    let (prompt, system_prompt) = (
      format!("<QUERY>\n{}\n</QUERY>", combined_code),
      fs::read_to_string("prompts/fill.txt")?,
    );

    let new_code = current_code.replace(
      hole,
      &extract_completion(&ask(&system_prompt, &prompt, &model)?)?,
    );

    let diff = TextDiff::from_lines(current_code, &new_code);

    println!("Proposed changes for hole '{}':", hole);

    for change in diff.iter_all_changes() {
      let (sign, style) = match change.tag() {
        ChangeTag::Delete => ("-", console::Style::new().red()),
        ChangeTag::Insert => ("+", console::Style::new().green()),
        ChangeTag::Equal => (" ", console::Style::new()),
      };

      print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
    }

    if Confirm::with_theme(&ColorfulTheme::default())
      .with_prompt("Do you want to apply these changes?")
      .default(true)
      .interact()?
    {
      println!("Changes applied.");
      Ok(new_code)
    } else {
      println!("Changes discarded.");
      Ok(current_code.to_string())
    }
  }
}

fn import_context_files(code: &str, main_file: &str) -> Result<String> {
  let regex = Regex::new(r"//\./(.*?)//")?;

  let mut updated_code = code.to_string();

  for capture in regex.captures_iter(code) {
    let import_path = Path::new(main_file).parent().unwrap().join(&capture[1]);
    if import_path.exists() {
      let import_text = fs::read_to_string(&import_path)?;
      println!("Importing file: {}", &capture[1]);
      updated_code =
        updated_code.replace(&capture[0], &format!("\n{}", import_text));
    } else {
      println!("Error: Import file not found: {}", &capture[1]);
      return Err(anyhow!("Import file not found: {:?}", import_path));
    }
  }

  Ok(updated_code)
}

fn token_count(text: &str) -> usize {
  text.split_whitespace().count()
}

fn find_holes(text: &str) -> Vec<String> {
  let regex = Regex::new(r"\{\{\w+\}\}").unwrap();

  let holes: Vec<String> = regex
    .find_iter(text)
    .map(|m| m.as_str().to_string())
    .collect();

  if holes.is_empty() && text.contains("??") {
    vec!["??".to_string()]
  } else {
    holes
  }
}

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

fn extract_completion(answer: &str) -> Result<String> {
  let regex = Regex::new(r"<COMPLETION>([\s\S]*?)</COMPLETION>")?;

  regex
    .captures(answer)
    .and_then(|cap| cap.get(1))
    .map(|m| m.as_str().to_string())
    .ok_or_else(|| {
      anyhow!("Could not find <COMPLETION> tags in the AI's response")
    })
}

fn main() -> Result<()> {
  dotenv::dotenv().ok();

  Arguments::parse().run()
}
