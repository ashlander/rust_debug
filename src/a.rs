//mod b; // FIXME
//use crate::hi_b;
use crate::b::hi_b;

pub fn hi_a() {
    hi_b();
    println!("[library] Hi from A!");
}
