use clap::Parser;

/// Bevy with avian
///
#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Verbosity level: 0=warn, 1=info, 2=debug
    #[arg(long = "verbosity", default_value_t = 1)]
    pub verbosity: u8,
}

impl Args {
    /// Validate numeric constraints that clap doesn't enforce here.
    fn validate(&self) -> anyhow::Result<()> {
        if self.verbosity > 2 {
            anyhow::bail!("--verbosity must be in 0..=2");
        }
        Ok(())
    }
}

/// Parse CLI args in one place so main.rs does not need clap in scope.
pub fn parse() -> Args {
    let args = Args::parse();
    // Fail-fast on invalid values
    if let Err(e) = args.validate() {
        // Print a friendly error and exit with non-zero, matching clap behavior
        eprintln!("error: {}", e);
        std::process::exit(2);
    }
    args
}
