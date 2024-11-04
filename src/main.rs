mod a;
use a::*;

mod b;
use b::*;

fn main() {
    println!("[bin] Hi from Main!");
    hi_a();
    hi_b();
    std::process::exit(0)
}
