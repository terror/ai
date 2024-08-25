use super::*;

#[derive(Debug, Clone)]
pub(crate) enum Model {
  GPT4,
  GPT4_32K,
  GPT3_5Turbo,
  GPT3_5turbo16k,
}

impl FromStr for Model {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "gpt-4" => Ok(Model::GPT4),
      "gpt-4-32k" => Ok(Model::GPT4_32K),
      "gpt-3.5-turbo" => Ok(Model::GPT3_5Turbo),
      "gpt-3.5-turbo-16k" => Ok(Model::GPT3_5turbo16k),
      _ => Err(format!("Invalid model: {}", s)),
    }
  }
}

impl ToString for Model {
  fn to_string(&self) -> String {
    match self {
      Model::GPT4 => "gpt-4".to_string(),
      Model::GPT4_32K => "gpt-4-32k".to_string(),
      Model::GPT3_5Turbo => "gpt-3.5-turbo".to_string(),
      Model::GPT3_5turbo16k => "gpt-3.5-turbo-16k".to_string(),
    }
  }
}

impl Into<Provider> for Model {
  fn into(self) -> Provider {
    match self {
      Model::GPT4
      | Model::GPT4_32K
      | Self::GPT3_5Turbo
      | Self::GPT3_5turbo16k => Provider::OpenAI,
    }
  }
}

impl Model {
  pub(crate) fn ask(self, system_prompt: &str, prompt: &str) -> Result<String> {
    let service = Into::<Provider>::into(self.clone());

    let config = Config::load()?;

    ensure!(
      config.has_key(&service),
      "no api key set for {}",
      service.to_string()
    );

    let client = Client::new();

    match service {
      Provider::OpenAI => {
        let response = client
          .post(service.url())
          .header(
            "Authorization",
            format!("Bearer {}", config.open_ai_api_key),
          )
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
            .ok_or(anyhow!("could not get completion from ai response"))?
            .to_string(),
        )
      }
      _ => todo!(),
    }
  }
}
