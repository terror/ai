use {
  super::*,
  crate::subcommand::{fill::Fill, set_key::SetKey},
};

mod fill;
mod set_key;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  Fill(Fill),
  SetKey(SetKey),
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result<()> {
    match self {
      Subcommand::Fill(fill) => fill.run(options),
      Subcommand::SetKey(set_key) => set_key.run(options),
    }
  }
}
