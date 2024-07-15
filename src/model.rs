use super::*;

#[derive(Debug, Clone)]
pub(crate) enum OpenAIModel {
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

impl OpenAIModel {
  pub(crate) fn ask(self, system_prompt: &str, prompt: &str) -> Result<String> {
    let client = Client::new();

    let api_key =
      std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let url = "https://api.openai.com/v1/chat/completions";

    let response = client
      .post(url)
      .header("Authorization", format!("Bearer {}", api_key))
      .json(&json!({
          "model": self.to_string(),
          "messages": [
              {"role": "system", "content": system_prompt},
              {"role": "user", "content": prompt}
          ]
      }))
      .send()?;

    let json = response.json::<serde_json::Value>()?;

    Ok(
      json["choices"][0]["message"]["content"]
        .as_str()
        .ok_or(anyhow!("Could not get completion from AI response"))?
        .to_string(),
    )
  }
}
