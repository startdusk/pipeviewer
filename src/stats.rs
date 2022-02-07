use std::io::Result;

use crossbeam::channel::Receiver;

pub fn stats_loop(slient: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;
        if !slient {
            eprint!("\r{}", total_bytes);
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
