extern crate jitter;

use jitter::jit::get_jit;
fn main(){
    let fun = get_jit(include_bytes!("../target/temp.bin").iter().cloned().collect());
    fun();
}
