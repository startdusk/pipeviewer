pub fn stats(slient: bool, num_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += num_read;
    if !slient {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!()
        }
    }
}
