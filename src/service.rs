use super::*;

#[derive(Debug, Clone)]
pub(crate) enum Service {
  OpenAI,
  Anthropic,
}

impl FromStr for Service {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "openai" => Ok(Service::OpenAI),
      "anthropic" => Ok(Service::Anthropic),
      _ => Err(format!("Invalid service: {}", s)),
    }
  }
}

impl ToString for Service {
  fn to_string(&self) -> String {
    match self {
      Service::OpenAI => "openai".to_string(),
      Service::Anthropic => "anthropic".to_string(),
    }
  }
}

impl Service {
  pub(crate) fn url<'a>(&self) -> &'a str {
    match self {
      Service::OpenAI => "https://api.openai.com/v1/chat/completions",
      _ => todo!()
    }
  }
}
