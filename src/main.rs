#![warn(clippy::pedantic)]

mod enigma;

mod prelude {
    pub use crate::enigma::*;
}

fn main() {
    println!("Hello, world!");
}
