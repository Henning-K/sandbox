extern crate sandbox;

use sandbox::networking_tests;
use sandbox::base64;

fn main() {
    println!("{}", networking_tests::testing_output());

    let man = "Man";
    println!("String: {}\nEncoded: {}", man, base64::encode(man));

    let leviathan = "Man is distinguished, not only by his reason, but by this singular passion \
                     from other animals, which is a lust of the mind, that by a perseverance of \
                     delight in the continued and indefatigable generation of knowledge, exceeds \
                     the short vehemence of any carnal pleasure.";

    println!("String: {}\nEncoded: {}",
             leviathan,
             base64::encode(leviathan));
}
