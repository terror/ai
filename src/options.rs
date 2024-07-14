use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Options {
  #[clap(
    long,
    short,
    help = "Model name (default: 'gpt-3.5-turbo')",
    default_value = "gpt-3.5-turbo"
  )]
  pub(crate) model: OpenAIModel,
}
