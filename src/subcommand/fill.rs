use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Fill {
  #[clap(help = "Input file path")]
  file: String,
  #[clap(long, short, help = "Context file paths", num_args = 0..)]
  context: Vec<PathBuf>,
}

impl Fill {
  pub(crate) fn run(self, options: Options) -> Result<()> {
    let code = fs::read_to_string(&self.file)?;

    let mut context_code = String::new();

    for context_file in &self.context {
      let content = fs::read_to_string(context_file)?;
      context_code.push_str(&content);
      context_code.push('\n');
    }

    let combined_code =
      Self::import(&format!("{}\n{}", context_code, code), &self.file)?;

    let tokens = combined_code.split_whitespace().count();

    let holes = Regex::new(r"\{\{\w+\}\}")
      .unwrap()
      .find_iter(&combined_code)
      .map(|m| m.as_str().to_string())
      .collect::<Vec<String>>();

    println!("Analysis complete:");

    if holes.is_empty() {
      println!("No holes found in the code.");
      return Ok(());
    }

    println!("  - Holes found: {:?}", holes);
    println!("  - Total token count: {}", tokens);
    println!("  - Using model: {}", options.model.to_string());

    let mut updated_file_code = code.clone();

    for (index, hole) in holes.iter().enumerate() {
      println!(
        "\nProcessing hole {} of {}: '{}'",
        index + 1,
        holes.len(),
        hole
      );
      updated_file_code = Self::process_hole(
        hole,
        &combined_code,
        &updated_file_code,
        options.model.clone(),
      )?;
    }

    fs::write(&self.file, &updated_file_code)?;

    println!("\nFile successfully updated: {}", self.file);

    Ok(())
  }

  fn process_hole(
    hole: &str,
    combined_code: &str,
    current_code: &str,
    model: OpenAIModel,
  ) -> Result<String> {
    println!("Generating completion for hole: '{}'...", hole);

    let (prompt, system_prompt) = (
      format!("<QUERY>\n{}\n</QUERY>", combined_code),
      fs::read_to_string("prompts/fill.txt")?,
    );

    let new_code = current_code.replace(
      hole,
      &Self::extract_completion(&model.ask(&system_prompt, &prompt)?)?,
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

  fn import(code: &str, main_file: &str) -> Result<String> {
    let regex = Regex::new(r"//\./(.*?)//")?;

    let mut updated_code = code.to_string();

    for capture in regex.captures_iter(code) {
      let import_path =
        Path::new(main_file).parent().unwrap().join(&capture[1]);
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
}
