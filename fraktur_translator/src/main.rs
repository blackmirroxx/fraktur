use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    // Mapping for uppercase Fraktur letters
    let uppercase_map: HashMap<char, char> = [
        ('A', '\u{1D504}'),
        ('B', '\u{1D505}'),
        ('D', '\u{1D507}'),
        ('E', '\u{1D508}'),
        ('F', '\u{1D509}'),
        ('G', '\u{1D50A}'),
        ('J', '\u{1D50D}'),
        ('K', '\u{1D50E}'),
        ('L', '\u{1D50F}'),
        ('M', '\u{1D510}'),
        ('N', '\u{1D511}'),
        ('O', '\u{1D512}'),
        ('P', '\u{1D513}'),
        ('Q', '\u{1D514}'),
        ('S', '\u{1D516}'),
        ('T', '\u{1D517}'),
        ('U', '\u{1D518}'),
        ('V', '\u{1D519}'),
        ('W', '\u{1D51A}'),
        ('X', '\u{1D51B}'),
        ('Y', '\u{1D51C}'),
        ('Z', '\u{1D51D}'),
    ].iter().cloned().collect();

    // Mapping for lowercase Fraktur letters
    let lowercase_map: HashMap<char, char> = [
        ('a', '\u{1D51E}'),
        ('b', '\u{1D51F}'),
        ('c', '\u{1D520}'),
        ('d', '\u{1D521}'),
        ('e', '\u{1D522}'),
        ('f', '\u{1D523}'),
        ('g', '\u{1D524}'),
        ('h', '\u{1D525}'),
        ('i', '\u{1D526}'),
        ('j', '\u{1D527}'),
        ('k', '\u{1D528}'),
        ('l', '\u{1D529}'),
        ('m', '\u{1D52A}'),
        ('n', '\u{1D52B}'),
        ('o', '\u{1D52C}'),
        ('p', '\u{1D52D}'),
        ('q', '\u{1D52E}'),
        ('r', '\u{1D52F}'),
        ('s', '\u{1D530}'),
        ('t', '\u{1D531}'),
        ('u', '\u{1D532}'),
        ('v', '\u{1D533}'),
        ('w', '\u{1D534}'),
        ('x', '\u{1D535}'),
        ('y', '\u{1D536}'),
        ('z', '\u{1D537}'),
    ].iter().cloned().collect();

    let mut output = String::new();
    let mut chars = buffer.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            'ä' => {
                output.push('\u{1D51E}'); // a
                output.push('\u{1D522}'); // e
            },
            'Ä' => {
                output.push('\u{1D504}'); // A
                output.push('\u{1D508}'); // E
            },
            'ö' => {
                output.push('\u{1D52C}'); // o
                output.push('\u{1D522}'); // e
            },
            'Ö' => {
                output.push('\u{1D512}'); // O
                output.push('\u{1D508}'); // E
            },
            'ü' => {
                output.push('\u{1D532}'); // u
                output.push('\u{1D522}'); // e
            },
            'Ü' => {
                output.push('\u{1D518}'); // U
                output.push('\u{1D508}'); // E
            },
            'ß' => {
                output.push('\u{1D530}'); // s
                output.push('\u{1D537}'); // z
            },
            'å' => {
                output.push('\u{1D51E}'); // a
                output.push('\u{1D51E}'); // a
            },
            'Å' => {
                output.push('\u{1D504}'); // A
                output.push('\u{1D504}'); // A
            },
            'é' => {
                output.push('\u{1D522}'); // e
                output.push('\u{1D522}'); // e
            },
            'É' => {
                output.push('\u{1D508}'); // E
                output.push('\u{1D508}'); // E
            },
            'è' | 'ê' => {
                output.push('\u{1D51E}'); // a
                output.push('\u{1D522}'); // e
            },
            'È' | 'Ê' => {
                output.push('\u{1D504}'); // A
                output.push('\u{1D508}'); // E
            },
            _ => {
                if let Some(fraktur) = uppercase_map.get(&c) {
                    output.push(*fraktur);
                } else if let Some(fraktur) = lowercase_map.get(&c) {
                    output.push(*fraktur);
                } else {
                    output.push(c);
                }
            }
        }
    }

    print!("{}", output);
    Ok(())
}

