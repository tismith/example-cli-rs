//standard includes
extern crate example_cli_rs;
use example_cli_rs::*;

fn main() -> Result<(), exitfailure::ExitFailure> {
    let mut config = utils::cmdline::parse_cmdline();
    config.module_path = Some(module_path!().into());
    utils::logging::configure_logger(&config)?;
    Ok(())
}
