use libbpf_rs::query::ProfInfoIter;

fn main() {
    let mut iter = ProgInfoIter::default();
    for prog in iter {
        println!("{}", prog.name.to_string_lossy());
    }
}
