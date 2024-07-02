#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
use libbpf_rs::query::{ProgInfoIter, ProgInfoQueryOptions};

fn main() {
    do_main();
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn do_main() {
    let iter = ProgInfoIter::with_query_opts(ProgInfoQueryOptions::default().include_all());
    for prog in iter {
        println!("{} (type: {})", prog.name.to_string_lossy(), prog.ty);
    }
}

#[cfg(any(not(target_os = "linux"), not(target_arch = "x86_64")))]
fn do_main() {
    eprintln!("This operating system and/or architecture is not supported.");
}
