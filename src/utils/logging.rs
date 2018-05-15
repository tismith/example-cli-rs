use stderrlog;
use utils::types;

///This sets up logging, and takes the output from the commandline
///options
pub fn configure_logger(config: &types::Settings) {
    let mut logger = stderrlog::new();

    logger
        .quiet(config.quiet)
        .verbosity(config.verbosity)
        .timestamp(config.timestamp);

    if let Some(ref module_path) = config.module_path {
        logger.module(module_path.clone());
    }

    logger.init().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_configure_logger() {
        //test that we don't panic creating a default logger
        configure_logger(&Default::default());
    }
}
