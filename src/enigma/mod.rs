mod plugboard;
mod reflector;
mod rotor;

struct Enigma {
    left_rotor: rotor::Rotor,
    middle_rotor: rotor::Rotor,
    right_rotor: rotor::Rotor,

    reflector: reflector::Reflector,

    plugboard: plugboard::Plugboard
}

impl Enigma {
    pub fn new(rotors: [&'static str;3], reflector: &'static str, rotor_positions: [u8;3], ring_settings: [u8;3], plugboard_connections: &'static str) -> Self {
        let left_rotor = rotor::Rotor::new(rotors[0], rotor_positions[0], ring_settings[0]);
        let middle_rotor = rotor::Rotor::new(rotors[1], rotor_positions[1], ring_settings[1]);
        let right_rotor = rotor::Rotor::new(rotors[2], rotor_positions[2], ring_settings[2]);
        let reflector = reflector::Reflector::new(reflector);
        let plugboard = plugboard::Plugboard::new(plugboard_connections);
        Self {
            left_rotor,
            middle_rotor,
            right_rotor,
            reflector,
            plugboard
        }
    }
    pub fn rotate(&mut self) {
        if self.middle_rotor.at_notch() {
            self.middle_rotor.turnover();
            self.left_rotor.turnover();
        } else if self.right_rotor.at_notch() {
            self.middle_rotor.turnover();
        }
        self.right_rotor.turnover();
    }
    pub fn encrypt(&mut self, mut character: u8) -> u8 {
        self.rotate();
        // In the plugboard
        character = self.plugboard.forward(character);
        // Right to left
        character = self.right_rotor.forward(character);
        character = self.middle_rotor.forward(character);
        character = self.left_rotor.forward(character);
        // Reflector
        character = self.reflector.forward(character);
        // Left to right
        character = self.left_rotor.backward(character);
        character = self.middle_rotor.backward(character);
        character = self.right_rotor.backward(character);
        // Out the plugboard
        self.plugboard.forward(character)
    }
    pub fn encrypt_char(&mut self, c: char) -> char {
        (self.encrypt(c as u8 -65) + 65) as char
    }

    pub fn encrypt_text(&mut self, input: &[char]) -> Vec<char> {
        let mut output = Vec::new();
        for character in input.iter() {
            output.push(self.encrypt_char(*character));
        }
        output
    }
}

pub fn decode_wiring(encoding: &'static str) -> Vec<u8> {
    let char_wiring = encoding.as_bytes();
    let mut wiring: Vec<u8> = Vec::new();
    for char in char_wiring.iter() {
        wiring.push(*char - 65);
    }
    wiring
}

fn inverse_wiring(mut wiring: Vec<u8>) -> Vec<u8> {
    wiring.reverse();
    wiring
}

fn encipher(k: u8, pos: u8, ring: u8, map: &[u8]) -> u8 {
    let shift = pos - ring;
    (map[((k + shift + 26) % 26) as usize] - shift + 26) % 26
}
