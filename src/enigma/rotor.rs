use super::{decode_wiring, inverse_wiring, encipher};
pub struct Rotor {
    name: &'static str,
    forward_wiring: Vec<u8>,
    backward_wiring: Vec<u8>,
    rotor_position: u8,
    notch_position: u8,
    ring_setting: u8,
}

impl Rotor {
    pub fn new(name: &'static str, rotor_position: u8, ring_setting: u8) -> Self {
        let mut notch_position = 0;
        let mut encoding = "";
        match name {
            "I" => {
                encoding = "EKMFLGDQVZNTOWYHXUSPAIBRCJ";
                notch_position = 16;
            },
            "II" => {
                encoding = "AJDKSIRUXBLHWTMCQGZNPYFVOE";
                notch_position = 4;
            },
            "III" => {
                encoding = "BDFHJLCPRTXVZNYEIWGAKMUSQO";
                notch_position = 21;
            },
            "IV" => {
                encoding = "ESOVPZJAYQUIRHXLNFTGKDCMWB";
                notch_position = 9;
            },
            "V" => {
                encoding = "VZBRGITYUPSDNHLXAWMJQOFECK";
                notch_position = 25;
            },
            "VI" => {
                encoding = "JPGVOUMFYQBENHZRDKASXLICTW";
            },
            "VII" => {
                encoding = "NZJHGRCXMYSWBOUFAIVLPEKQDT";
            },
            "VIII" => {
                encoding = "FKQHTLXOCBJSPDZRAMEWNIUYGV";
            },
            _ => {}
        }
        Self {
            name,
            forward_wiring: decode_wiring(encoding),
            backward_wiring: inverse_wiring(decode_wiring(encoding)),
            rotor_position,
            notch_position,
            ring_setting
        }
    }
    pub fn get_name(&self) -> &str {
        self.name
    }
    pub fn rotor_position(&self) -> u8 { self.rotor_position }
    pub fn forward(&self, amount: u8) -> u8 {
        encipher(amount, self.rotor_position, self.ring_setting, &self.forward_wiring )
    }
    pub fn backward(&self, amount: u8) -> u8 {
        encipher(amount, self.rotor_position, self.ring_setting, &self.backward_wiring)
    }
    pub fn at_notch(&self) -> bool {
        match self.name {
            "VI" | "VII" | "VIII" => self.rotor_position == 12 || self.rotor_position == 25,
            _ => self.notch_position == self.rotor_position
        }
    }
    pub fn turnover(&mut self) {
        self.rotor_position = (self.rotor_position+1) % 26;
    }
}

