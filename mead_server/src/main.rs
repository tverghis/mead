use libbpf_rs::query::{ProgInfoIter, ProgInfoQueryOptions};

fn main() {
    let iter = ProgInfoIter::with_query_opts(ProgInfoQueryOptions::default().include_all());
    for prog in iter {
        println!("{} (type: {})", prog.name.to_string_lossy(), prog.ty);
    }
}
