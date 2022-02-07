use clap::{App, Arg};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub slient: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("pipeviewer")
            .arg(Arg::new("infile").help("Read from a file instead of stdin"))
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .takes_value(true)
                    .help("Write output to a file instead of stdout"),
            )
            .arg(Arg::new("slient").short('s').long("slient"))
            .get_matches();
        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let slient = if matches.is_present("slient") {
            true
        } else {
            !env::var("PV_SLIENT").unwrap_or_default().is_empty()
        };

        Self {
            infile,
            outfile,
            slient,
        }
    }
}
