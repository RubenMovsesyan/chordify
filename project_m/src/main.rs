use std::io;

mod key;

const KEYS : &str = "./res/keys.txt";

fn main() {
    println!("This is the music generator!");


    let keys: Vec<key::KeyData> = key::read_key_file(KEYS);

    loop {
        println!("Which key would you like to see? (type q to quit)");
        let set_key: u16;
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
        
        // I know this is not the best way but the trim method doesn't work
        input.pop();
        let _ = input.to_ascii_lowercase();
        
        match input.as_str() {
            "a" => set_key  = key::A,
            "as" => set_key = key::AS,
            "b" => set_key  = key::B,
            "c" => set_key  = key::C,
            "cs" => set_key = key::CS,
            "d" => set_key  = key::D,
            "ds" => set_key = key::DS,
            "e" => set_key  = key::E,
            "f" => set_key  = key::F,
            "fs" => set_key = key::FS,
            "g" => set_key  = key::G,
            "gs" => set_key = key::GS,
            "q" => break,
            _ => set_key    = key::A,
        }

        for k in &keys {
            k.print(set_key);
        }
    }
}