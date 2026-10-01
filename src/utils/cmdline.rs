use clap::{self, Arg, ArgAction, Command};
use utils::types;

pub fn parse_cmdline() -> types::Settings {
    let matches = matcher().get_matches();
    match parse(&matches) {
        Ok(s) => s,
        Err(e) => e.exit(),
    }
}

fn matcher() -> Command {
    Command::new("example-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::new("verbosity")
                .short('v')
                .action(ArgAction::Count)
                .help("Increase message verbosity, maximum 4"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Silence all output"),
        )
        .arg(
            Arg::new("timestamp")
                .short('t')
                .long("timestamp")
                .value_parser(["none", "sec", "ms", "ns"])
                .help("prepend log lines with a timestamp"),
        )
}

fn parse(matches: &clap::ArgMatches) -> Result<types::Settings, clap::Error> {
    let verbosity = matches.get_count("verbosity") as usize;
    if verbosity > 4 {
        return Err(clap::Error::raw(
            clap::error::ErrorKind::InvalidValue,
            "invalid number of 'v' flags",
        ));
    }
    let quiet = matches.get_flag("quiet");
    let timestamp = match matches.get_one::<String>("timestamp").map(String::as_str) {
        Some("ns") => types::Timestamp::Nanosecond,
        Some("ms") => types::Timestamp::Microsecond,
        Some("sec") => types::Timestamp::Second,
        Some("none") | None => types::Timestamp::Off,
        Some(_) => unreachable!("clap validates timestamp values"),
    };

    Ok(types::Settings {
        verbosity,
        quiet,
        timestamp,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_too_much_verbosity() {
        let m = matcher().try_get_matches_from(vec!["", "-vvvvv"]).unwrap();
        assert!(parse(&m).is_err());
    }

    #[test]
    fn test_just_enough_verbosity() {
        let m = matcher().try_get_matches_from(vec!["", "-vvv"]).unwrap();
        let s = parse(&m).unwrap();

        assert_eq!(s.verbosity, 3);
    }

    #[test]
    fn test_quiet() {
        let m = matcher().try_get_matches_from(vec!["", "-q"]).unwrap();
        assert!(parse(&m).unwrap().quiet);
    }

    #[test]
    fn test_timestamps() {
        let m = matcher()
            .try_get_matches_from(vec!["", "-t", "sec"])
            .unwrap();
        let s = parse(&m).unwrap();

        match s.timestamp {
            types::Timestamp::Second => (),
            _ => panic!("unexpected parse"),
        }
    }

    #[test]
    fn test_bogus_timestamps() {
        assert!(
            matcher()
                .try_get_matches_from(vec!["", "-t", "bogus"])
                .is_err()
        );
    }
}
