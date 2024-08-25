use super::*;

#[derive(Debug, Clone)]
pub(crate) enum Provider {
  OpenAI,
  Anthropic,
}

impl FromStr for Provider {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "openai" => Ok(Provider::OpenAI),
      "anthropic" => Ok(Provider::Anthropic),
      _ => Err(format!("Invalid service: {}", s)),
    }
  }
}

impl ToString for Provider {
  fn to_string(&self) -> String {
    match self {
      Provider::OpenAI => "openai".to_string(),
      Provider::Anthropic => "anthropic".to_string(),
    }
  }
}

impl Provider {
  pub(crate) fn url<'a>(&self) -> &'a str {
    match self {
      Provider::OpenAI => "https://api.openai.com/v1/chat/completions",
      _ => todo!(),
    }
  }
}
