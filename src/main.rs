use clap::Parser;

#[derive(Debug, Parser)]
struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand
}

impl Arguments {
  fn run(self) -> Result {
    self.subcommand.run()
  }
}

#[derive(Debug, Parser)]
enum Subcommand {
  Fill(Fill)
}

impl Subcommand {
  fn run(self) -> Result {
    match self {
      Subcommand::Fill(fill) => fill.run()
    }
  }
}

#[derive(Debug, Parser)]
struct Fill;

impl Fill {
  fn run(self) -> Result {
    println!("Fill");
    Ok(())
  }
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
  }
}
