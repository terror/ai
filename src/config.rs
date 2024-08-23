use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
  pub(crate) anthropic_api_key: String,
  pub(crate) open_ai_api_key: String,
}

impl Default for Config {
  fn default() -> Self {
    Config {
      anthropic_api_key: String::new(),
      open_ai_api_key: String::new(),
    }
  }
}

impl Config {
  const CONFIG_FILE_NAME: &'static str = "config.json";

  pub(crate) fn has_key(&self, service: Service) -> bool {
    match service {
      Service::Anthropic => !self.anthropic_api_key.is_empty(),
      Service::OpenAI => !self.open_ai_api_key.is_empty(),
    }
  }

  pub(crate) fn load() -> Result<Self> {
    let directory = BaseDirectories::with_prefix("ai")?;

    let path = directory.get_config_file(Self::CONFIG_FILE_NAME);

    if !path.exists() {
      let config = Config::default();
      config.save()?;
      return Ok(config);
    }

    Ok(serde_json::from_str::<Config>(&fs::read_to_string(&path)?)?)
  }

  pub(crate) fn save(&self) -> Result {
    let directory = BaseDirectories::with_prefix("ai")?;

    let config_path = directory.place_config_file(Self::CONFIG_FILE_NAME)?;

    fs::write(&config_path, serde_json::to_string_pretty(self)?)?;

    Ok(())
  }

  pub(crate) fn set_key(&mut self, service: Service, api_key: String) {
    match service {
      Service::Anthropic => self.anthropic_api_key = api_key,
      Service::OpenAI => self.open_ai_api_key = api_key,
    }
  }
}
