use std::io::Result;
use std::sync::mpsc;
use std::thread;

use pipeviewer::{args::Args, read, stats, write};

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        slient,
    } = args;
    let (stats_tx, stats_rx) = mpsc::channel();
    let (write_tx, write_rx) = mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(slient, stats_rx, write_tx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed
    // `.join()` returns a `thread::Result<io::Result<()>>`
    let read_to_result = read_handle.join().unwrap();
    let stats_to_result = stats_handle.join().unwrap();
    let write_to_result = write_handle.join().unwrap();

    // Return an error if any threads returned an error
    read_to_result?;
    stats_to_result?;
    write_to_result?;
    Ok(())
}
