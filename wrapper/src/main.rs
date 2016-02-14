extern crate coki_jitter;

use coki_jitter::jit::get_jit;
pub fn main() {
    let fun = get_jit(include_bytes!("../temp.bin"));
    fun();
}
