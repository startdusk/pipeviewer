use std::io::Result;
use std::time::Instant;

use crossbeam::channel::Receiver;

pub fn stats_loop(slient: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    let start = Instant::now();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;
        if !slient {
            eprint!("\r{} {}", total_bytes, start.elapsed().as_secs());
        }
        if num_bytes == 0 {
            break;
        }
    }
    if !slient {
        eprintln!();
    }
    Ok(())
}
