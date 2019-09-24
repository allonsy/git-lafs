mod hash;
mod smudge;
mod clean;
mod repo;

const PROTOCOL_VERSION: &'static str="lafs-v1";

fn main() {
    println!("{}", hash::hash_file("sifted.png").unwrap());
    println!("{}", clean::clean_file("sifted.png").unwrap());
}
