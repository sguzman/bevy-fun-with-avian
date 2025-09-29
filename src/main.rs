mod args;
mod log;

fn main() -> anyhow::Result<()> {
    let args = args::parse();

    // Init colorized, timestamped logging
    log::init(log::level_from_verbosity(args.verbosity));

    Ok(())
}
