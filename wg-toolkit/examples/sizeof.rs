use indexmap::IndexMap;

fn main() {
    println!("sizeof : {}", std::mem::size_of::<IndexMap<String, usize>>());
}
