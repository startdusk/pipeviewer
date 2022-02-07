use std::io::Result;

use pipeviewer::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_bytes = 0;
    loop {
        let buffer = match read::read(&args.infile) {
            Ok(x) if x.is_empty() => break,
            Ok(x) => x,
            Err(_) => break,
        };
        stats::stats(args.slient, buffer.len(), &mut total_bytes, false);
        if !write::write(&args.outfile, &buffer)? {
            break;
        }
    }
    stats::stats(args.slient, 0, &mut total_bytes, true);
    Ok(())
}
