/// Entry point for encoding input strings into base64-encoded output strings.
pub fn encode(inp: &str) -> String {
    let base64_index = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                           .chars()
                           .collect::<Vec<char>>();
    let mut result = String::new();
    let mut it = inp.encode_utf16().map(|e| e as u32);

    loop {
        let triplet = vec![it.next(), it.next(), it.next()];
        match triplet[0] {
            None => break,
            Some(_) => {}
        }

        let loop_end = triplet.iter().any(|e| match *e {
            None => true,
            _ => false,
        }); // look if any of the elements is None and if so flip the bool switch so the loop will be broken out of after some final tasks.

        let mut bit_string = 0u32;
        for i in 0..2 {
            bit_string = bit_string | triplet[i].unwrap_or(0u32); // unwrap_or(some_value) unwraps the Option/Result and returns the value of Some(_) or the default some_value.
            bit_string <<= 8;
        }
        bit_string = bit_string | triplet[2].unwrap_or(0u32);

        let sextet3 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet2 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet1 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet0 = (bit_string & 0x3F) as usize;

        let lsb1 = match triplet[1] {
            None => '=',
            _ => base64_index[sextet2],
        };
        let lsb0 = match triplet[2] {
            None => '=',
            _ => base64_index[sextet3],
        };

        result = format!("{}{}{}{}{}",
                         result,
                         base64_index[sextet0],
                         base64_index[sextet1],
                         lsb1,
                         lsb0);
        if loop_end {
            break;
        }
    }
    result
}

/// Entry point for reversing base64-encoded input strings back to
pub fn decode(inp: &str) -> String {
    let base64_index = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                           .chars()
                           .collect::<Vec<char>>();
    let inp = match inp.len() % 4 {
        2 => format!("{}==", inp),
        3 => format!("{}=", inp),
        _ => inp.to_owned(),
    };

    let mut it = inp.as_str().chars().map(|e| e as u8);

    let mut result = String::new();
    loop {
        let mut quartet_: Vec<Option<u8>> = vec![];

        for _ in 0..4 {
            quartet_.push(it.next());
        }

        if quartet_.iter().any(|e| match *e {
            None => true,
            _ => false,
        }) {
            break;
        }

        let quartet = quartet_.iter().map(|e| (*e).unwrap_or(0u8)).collect::<Vec<u8>>();

        let mut bit_string = 0u32;
        for i in 0..4 {
            bit_string = bit_string |
                         base64_index.iter()
                                     .position(|&x| x == (quartet[i] as char))
                                     .unwrap_or(0usize) as u32;
            if i != 3 {
                bit_string <<= 6;
            }
        }

        let octet2 = match quartet[3] {
            0x3D => 0x0,
            _ => (bit_string & 0xFF) as u8,
        };
        bit_string >>= 8;
        let octet1 = match quartet[2] {
            0x3D => 0x0,
            _ => (bit_string & 0xFF) as u8,
        };
        bit_string >>= 8;
        let octet0 = (bit_string & 0xFF) as u8;

        let (octet0, octet1, octet2) = (octet0 as char, octet1 as char, octet2 as char);

        result = match (octet1, octet2) {
            ('\0', '\0') => format!("{}{}", result, octet0),
            (_, '\0') => format!("{}{}{}", result, octet0, octet1),
            _ => format!("{}{}{}{}", result, octet0, octet1, octet2),
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{encode, decode};

    fn leviathan() -> &'static str {
        "Man is distinguished, not only by his reason, but by this singular passion from other \
         animals, which is a lust of the mind, that by a perseverance of delight in the continued \
         and indefatigable generation of knowledge, exceeds the short vehemence of any carnal \
         pleasure."
    }

    fn leviathan_b64() -> &'static str {
        "TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2Ugb2YgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4="
    }

    #[test]
    fn encode_man() {
        assert_eq!("TWFu".to_owned(), encode("Man"));
    }


    #[test]
    fn encode_leviathan() {
        assert_eq!(leviathan_b64(), encode(leviathan()));
    }

    #[test]
    fn decode_man() {
        assert_eq!("Man".to_owned(), decode("TWFu"));
    }

    #[test]
    fn decode_leviathan() {
        assert_eq!(leviathan(), decode(leviathan_b64()));
    }
}
