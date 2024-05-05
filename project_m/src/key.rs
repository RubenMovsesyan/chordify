use std::fs;

pub const IS_MINOR: u8  = 0x10;
pub const IS_DIM: u8    = 0x20;

pub const A: u16        = 0x0001;
pub const AS: u16       = 0x0002;
pub const B: u16        = 0x0004;
pub const C: u16        = 0x0008;
pub const CS: u16       = 0x0010;
pub const D: u16        = 0x0020;
pub const DS: u16       = 0x0040;
pub const E: u16        = 0x0080;
pub const F: u16        = 0x0100;
pub const FS: u16       = 0x0200;
pub const G: u16        = 0x0400;
pub const GS: u16       = 0x0800;

pub struct KeyData {
    name: String,
    chord_dat: [u8; 7],
}

impl Default for KeyData {
    fn default() -> Self {
        Self {
            name: String::from(""),
            chord_dat: [0, 0 ,0 ,0 ,0 ,0 ,0],
        }
    }
}

impl KeyData {
    pub fn new(name: String, chord_dat: [u8; 7]) -> Self {
        Self {
            name: name,
            chord_dat: chord_dat,
        }
    }

    pub fn get_chord(&self, mut base: u16, index: usize) -> String {
        let mut output: String = Default::default();

        base = left_shift_12(base, (self.chord_dat[index] & 0x0F) as usize);

        match base {
            A  => output.push_str("A"),
            AS => output.push_str("A#"),
            B  => output.push_str("B"),
            C  => output.push_str("C"),
            CS => output.push_str("C#"),
            D  => output.push_str("D"),
            DS => output.push_str("D#"),
            E  => output.push_str("E"),
            F  => output.push_str("F"),
            FS => output.push_str("F#"),
            G  => output.push_str("G"),
            GS => output.push_str("G#"),
            _  => output.push_str("Err"),
        }

        let modifications = self.chord_dat[index];

        if modifications & IS_MINOR != 0 {
            output.push_str("m");
        }
        if modifications & IS_DIM != 0 {
            output.push_str("dim");
        }

        output
    }

    pub fn print(&self, base: u16) {
        let mut chords: Vec<String> = Default::default();

        for i in 0..self.chord_dat.len() {
            chords.push(self.get_chord(base, i));
        }

        print!("Scale: {}\nChords: ", self.get_chord(base, 0)[..1].to_string() + " " + &self.name);

        for chord in chords {
            print!("{chord} ");
        }
        println!();
    }
}

fn left_shift_12(val: u16, amount: usize) -> u16 {
    if amount == 0 {
        return val;
    }

    if val << amount > GS {
        return val >> 12 - amount;
    }

    val << amount
}

pub fn read_key_file(filename: &str) -> Vec<KeyData> {
    println!("In file {}", filename);

    let mut data: KeyData = Default::default();

    let mut keys: Vec<KeyData> = Vec::new();

    let mut key_active: bool = false;

    for line in fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>() {
        
        if key_active {
            let parts: Vec<String> = line.split_whitespace()
                            .map(|p| p.to_string())
                            .collect();
            
            // Name of key
            if parts[0].contains("name") {
                data.name = parts[1].clone();
            }
            // Scale of key
            if parts[0].contains("s") {
                let mut steps : u8 = 0;
                for i in 1..parts.len() {
                    data.chord_dat[i - 1] = steps;

                    if i >= parts.len() {
                        break;
                    }

                    if parts[i].contains("w") {
                        steps += 2;
                    } else {
                        steps += 1;
                    }
                }
            }
            // Chord types in key
            if parts[0].contains("c") {
                for i in 1..parts.len() {
                    if parts[i].contains("m") {
                        data.chord_dat[i - 1] |= IS_MINOR;
                    } else if parts[i].contains("d") {
                        data.chord_dat[i - 1] |= IS_DIM;
                    }
                }
            }

            // End of progression
            if parts[0].contains("p_end") {
                key_active = false;
                keys.push(KeyData::new(data.name.clone(), data.chord_dat));
            }
        }
        
        if line.contains("p_begin") {
            key_active = true;
            continue;
        }
    }

    keys
}
