// test4.rs
// This test covers the sections:
// - Modules
// - Macros

macro_rules! my_macro {
    ($val:expr) => { $val }
}

fn main() {
    if my_macro!("world!") != "Hello world!" {
        panic!("Oh no! Wrong output!");
    }
}
