use {super::*, crate::subcommand::fill::Fill};

mod fill;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  Fill(Fill),
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result<()> {
    match self {
      Subcommand::Fill(fill) => fill.run(options),
    }
  }
}
