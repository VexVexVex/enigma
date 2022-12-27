use crate::prelude::*;

pub(crate) struct Reflector {
    forward_wiring: Vec<u8>
}


impl Reflector {
    pub fn new(name: &'static str) -> Self {
        match name {
            "B" => Self {
                forward_wiring: decode_wiring("YRUHQSLDPXNGOKMIEBFZCWVJAT")
            },
            "C" => Self {
                forward_wiring: decode_wiring("FVPJIAOYEDRZXWGCTKUQSBNMHL")
            },
            _ => Self {
                forward_wiring: decode_wiring("ZYXWVUTSRQPONMLKJIHGFEDCBA")

            }
        }
    }
    pub fn forward(&self, amount: u8) -> u8 {
        self.forward_wiring[amount as usize]
    }
}