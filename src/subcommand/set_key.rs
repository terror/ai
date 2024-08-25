use super::*;

#[derive(Debug, Parser)]
pub(crate) struct SetKey {
  #[clap(
    default_value = "openai",
    help = "Service to set the API key for",
    required = true
  )]
  service: Provider,
  #[clap(required = true)]
  api_key: String,
}

impl SetKey {
  pub(crate) fn run(self, _options: Options) -> Result {
    let mut config = Config::load()?;
    config.set_key(self.service, self.api_key);
    config.save()?;
    Ok(())
  }
}
