use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Fill {
  #[clap(help = "Input file path")]
  file: String,
  #[clap(long, short, help = "Context file paths", num_args = 0..)]
  context: Vec<PathBuf>,
}

impl Fill {
  pub(crate) fn run(self, options: Options) -> Result {
    let code = fs::read_to_string(&self.file)?;

    let context_code = self
      .context
      .iter()
      .map(|path| fs::read_to_string(path))
      .collect::<Result<Vec<_>, _>>()?
      .join("\n");

    let combined_code =
      Self::add_context(&format!("{}\n{}", context_code, code), &self.file)?;

    let holes = Regex::new(r"\{\{\w+\}\}")?
      .find_iter(&combined_code)
      .map(|m| m.as_str().to_string())
      .collect::<Vec<String>>();

    if holes.is_empty() {
      println!("No holes found in the code.");
      return Ok(());
    }

    let tokens = combined_code.split_whitespace().count();

    println!("Holes found: {:?}", holes);
    println!("Total token count: {}", tokens);
    println!("Using model: {}", options.model.to_string());

    let updated_file_code = holes.into_iter().try_fold(code, |acc, hole| {
      Self::process_hole(&hole, &combined_code, &acc, options.model.clone())
    })?;

    fs::write(&self.file, &updated_file_code)?;

    println!("\nFile successfully updated: {}", self.file);

    Ok(())
  }

  fn extract_completion(answer: &str) -> Result<String> {
    Regex::new(r"<COMPLETION>([\s\S]*?)</COMPLETION>")?
      .captures(answer)
      .and_then(|cap| cap.get(1))
      .map(|m| m.as_str().to_string())
      .ok_or_else(|| {
        anyhow!("Could not find <COMPLETION> tags in the AI's response")
      })
  }

  fn add_context(code: &str, main: &str) -> Result<String> {
    Regex::new(r"//\./(.*?)//")?.captures_iter(code).try_fold(
      code.to_string(),
      |acc, capture| {
        let import_path = Path::new(main)
          .parent()
          .ok_or_else(|| anyhow!("Could not get parent directory"))?
          .join(&capture[1]);

        if import_path.exists() {
          Ok(acc.replace(
            &capture[0],
            &format!("\n{}", fs::read_to_string(&import_path)?),
          ))
        } else {
          Err(anyhow!("Import file not found: {:?}", import_path))
        }
      },
    )
  }

  fn process_hole(
    hole: &str,
    combined_code: &str,
    current_code: &str,
    model: Model,
  ) -> Result<String> {
    println!("Generating completion for hole: '{}'...", hole);

    let (prompt, system_prompt) = (
      format!("<QUERY>\n{}\n</QUERY>", combined_code),
      PROMPT_DIR
        .get_file("fill.txt")
        .ok_or(anyhow!("Failed to get file from prompt directory"))?
        .contents_utf8()
        .ok_or(anyhow!("Failed to get file contents from prompt directory"))?
        .to_string(),
    );

    let new_code = current_code.replace(
      hole,
      &Self::extract_completion(&model.ask(&system_prompt, &prompt)?)?,
    );

    let diff = TextDiff::from_lines(current_code, &new_code);

    println!("Proposed changes for hole '{}':", hole);

    diff.iter_all_changes().for_each(|change| {
      let (sign, style) = match change.tag() {
        ChangeTag::Delete => ("-", console::Style::new().red()),
        ChangeTag::Insert => ("+", console::Style::new().green()),
        ChangeTag::Equal => (" ", console::Style::new()),
      };

      print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
    });

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
      .with_prompt("Do you want to apply these changes?")
      .default(true)
      .interact()?;

    if confirmed {
      println!("Changes applied.");
      Ok(new_code)
    } else {
      println!("Changes discarded.");
      Ok(current_code.to_string())
    }
  }
}
