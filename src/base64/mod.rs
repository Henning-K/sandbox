
static BASE64_INDEX: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// Entry point for encoding input strings into base64-encoded output strings.
pub fn encode(inp: &str) -> String {
    let inp = inp;
    let mut result = String::new();
    let mut it = inp.chars().map(|e| e as u32);
    let inp_len_mod_3 = inp.len() % 3;

    // Replace the ugly pile of shit for loop in the following with something annalog to this:
    // let mut it = test_vec.iter();
    // loop {
    //     let one = it.next();
    //     let two = it.next();
    //     let three = it.next();
    //     let triplet = vec![one, two, three];
    //     if triplet.iter().any(|e| match *e {None => true, _ => false}) {
    //         break;
    //     }
    //     println!("{:?}", triplet);
    // }

    loop {
        let one = it.next();
        let two = it.next();
        let three = it.next();
        let triplet = vec![one, two, three];
        match one {
            None => break,
            Some(_) => {}
        }

        let loop_end = triplet.iter().any(|e| match *e {
            None => true,
            _ => false,
        }); // look if any of the elements is None and if so flip the bool switch so the loop will be broken out of after some final tasks.
        let mut bit_string = 0u32;
        bit_string = bit_string |
                     match one {
            Some(e) => e,
            _ => 0u32,
        };
        bit_string <<= 8;
        bit_string = bit_string |
                     match two {
            Some(e) => e,
            _ => 0u32,
        };
        bit_string <<= 8;
        bit_string = bit_string |
                     match three {
            Some(e) => e,
            _ => 0u32,
        };
        let sextet3 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet2 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet1 = (bit_string & 0x3F) as usize;
        bit_string >>= 6;
        let sextet0 = (bit_string & 0x3F) as usize;
        let mut temp = String::new();
        temp.push(BASE64_INDEX.chars().nth(sextet0).unwrap() as char);
        temp.push(BASE64_INDEX.chars().nth(sextet1).unwrap() as char);
        temp.push(BASE64_INDEX.chars().nth(sextet2).unwrap() as char);
        temp.push(BASE64_INDEX.chars().nth(sextet3).unwrap() as char);
        result.push_str(temp.as_str());
        if loop_end {
            break;
        }
    }
    result
}

/// Entry point for reversing base64-encoded input strings back to
pub fn decode(inp: &str) -> String {
    let mut result = String::new();
    let mut inp = inp.to_owned();
    match inp.len() % 4 {
        2 => {
            inp.push_str("==");
        }
        3 => {
            inp.push_str("=");
        }
        _ => {}
    };
    let sextets = inp.as_bytes().iter().map(|e| *e).collect::<Vec<u8>>();
    for (index, _) in sextets.iter().enumerate() {
        if index % 4 != 0 {
            continue;
        }
        let quartet = sextets.iter()
                             .skip(index)
                             .take(4)
                             .map(|e| *e as u32)
                             .collect::<Vec<u32>>();
        let mut bit_string = 0u32;
        bit_string = bit_string | quartet[0];
        bit_string <<= 6;
        bit_string = bit_string | quartet[1];
        bit_string <<= 6;
        bit_string = bit_string | quartet[2];
        bit_string <<= 6;
        bit_string = bit_string | quartet[3];
        let octet2 = (bit_string & 0xF) as u8;
        bit_string >>= 8;
        let octet1 = (bit_string & 0xF) as u8;
        bit_string >>= 8;
        let octet0 = (bit_string & 0xF) as u8;
        let mut temp = String::new();
        temp.push(octet0 as char);
        temp.push(octet1 as char);
        temp.push(octet2 as char);
        result.push_str(temp.as_str());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{encode, decode};

    static leviathan: &'static str = "Man is distinguished, not only by his reason, but by this \
                                      singular passion from other animals, which is a lust of the \
                                      mind, that by a perseverance of delight in the continued \
                                      and indefatigable generation of knowledge, exceeds the \
                                      short vehemence of any carnal  pleasure.";

    static leviathan_b64: &'static str = " TWFuIGlzIGRpc3Rpbmd1aXNoZWQsIG5vdCBvbmx5IGJ5IGhpcyByZWFzb24sIGJ1dCBieSB0aGlzIHNpbmd1bGFyIHBhc3Npb24gZnJvbSBvdGhlciBhbmltYWxzLCB3aGljaCBpcyBhIGx1c3Qgb2YgdGhlIG1pbmQsIHRoYXQgYnkgYSBwZXJzZXZlcmFuY2UgbYgZGVsaWdodCBpbiB0aGUgY29udGludWVkIGFuZCBpbmRlZmF0aWdhYmxlIGdlbmVyYXRpb24gb2Yga25vd2xlZGdlLCBleGNlZWRzIHRoZSBzaG9ydCB2ZWhlbWVuY2Ugb2YgYW55IGNhcm5hbCBwbGVhc3VyZS4=";

    #[test]
    fn encode_man() {
        assert_eq!("TWFu".to_owned(), encode("Man"));
    }


    #[test]
    fn encode_leviathan() {
        assert_eq!(leviathan_b64.to_owned(), encode(leviathan));
    }

    #[test]
    fn decode_man() {
        assert_eq!("Man".to_owned(), decode("TWFu"));
    }

    #[test]
    fn decode_leviathan() {
        assert_eq!(leviathan.to_owned(), decode(leviathan_b64));
    }
}
