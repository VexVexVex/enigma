use std::collections::HashSet;

pub(crate) struct Plugboard{
    wiring: [u8;26]
}

impl Plugboard {
    pub fn new(connections: &'static str) -> Self {
        Self {
            wiring: decode_plugboard(connections)
        }
    }
    pub fn forward(&self, amount: u8) -> u8 {
        self.wiring[amount as usize]
    }
}
fn identity_plugboard() -> [u8;26] {
    let mut mapping:[u8;26] = [0;26];
    for i in 0..26 {
        mapping[i] = i as u8;
    }
    mapping
}

fn decode_plugboard(plugboard: &'static str) -> [u8;26] {
    if plugboard == "" {
        return identity_plugboard();
    }
    let pairings = plugboard.split("[^a-zA-Z]");
    let mut plugged_characters: HashSet<u8> = HashSet::new();
    let mut mapping = identity_plugboard();

    for pair in pairings.into_iter() {
        if pair.len() != 2 {
            return identity_plugboard();
        }
        let mut c1= 0;
        let mut c2= 0;
        if let Some(i) = pair.chars().next() {
            c1 = i as u8
        }
        if let Some(i) = pair.chars().next() {
            c2 = i as u8
        }

        if plugged_characters.contains(&c1) || plugged_characters.contains(&c2) {
            return identity_plugboard();
        }
        plugged_characters.insert(c1);
        plugged_characters.insert(c2);

        mapping[c1 as usize] = c2;
        mapping[c2 as usize] = c1;
    }
    mapping
}

pub fn unplugged_characters(plugboard: &'static str) -> HashSet<u8> {
    let mut unplugged_characters: HashSet<u8> = HashSet::new();
    for i in 0..26 {
        unplugged_characters.insert(i);
    }
    if plugboard == "" {
        return unplugged_characters;
    }
    let pairings = plugboard.split("[^a-zA-Z]");
    let mut c1= 0;
    let mut c2= 0;
    for pair in pairings.into_iter() {
        if let Some(i) = pair.chars().next() {
            c1 = i as u8 - 65;
        }
        if let Some(i) = pair.chars().next() {
            c2 = i as u8 - 65;
        }
        unplugged_characters.remove(&c1);
        unplugged_characters.remove(&c2);
    }
    unplugged_characters
}