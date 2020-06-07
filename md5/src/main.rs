use std::convert::TryFrom;

fn main() {
    md5("");
}

// based on https://fr.wikipedia.org/wiki/MD5
fn md5(input: &str) -> &str {
    // generated using generate_constants function
    let constants = [
        28, 30, 4, 25,
        32, 9, 22, 33,
        14, 18, 33, 18,
        14, 33, 22, 9,
        32, 25, 5, 31,
        28, 0, 28, 30,
        4, 25, 32, 9,
        22, 33, 13, 18,
        33, 17, 14, 33,
        21, 10, 32, 25,
        5, 31, 28, 0,
        28, 30, 4, 26,
        32, 8, 22, 33,
        13, 18, 33, 17,
        14, 33, 21, 10,
        32, 25, 5, 31,
    ];

    let rotate = [
        7, 12, 17, 22,
        7, 12, 17, 22,
        7, 12, 17, 22,
        7, 12, 17, 22,

        5, 9, 14, 20,
        5, 9, 14, 20,
        5, 9, 14, 20,
        5, 9, 14, 20,

        4, 11, 16, 23,
        4, 11, 16, 23,
        4, 11, 16, 23,
        4, 11, 16, 23,

        6, 10, 15, 21,
        6, 10, 15, 21,
        6, 10, 15, 21,
        6, 10, 15, 21,
    ];

    let mut hash: [u32; 4] = [
        0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476
    ];

    let mut message = input.clone();
    let last_char = char::try_from((message.chars().last().unwrap().into() as u32) << 1 + 1).unwrap();
    message = &message[0..message.len() - 2] + last_char.to_string();


    ""
}

fn generate_constants() -> Vec<i32> {
    let mut k = Vec::with_capacity(64);

    for i in 0..64 {
        let value = (((i + 1) as f32).sin() * (2^32) as f32).abs().floor() as i32;
        k.push(value);
    }

    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        assert_eq!(md5("Et l’unique cordeau des trompettes marines"), "8747e564eb53cb2f1dcb9aae0779c2aa");
    }

    #[test]
    fn second_test() {
        assert_eq!(md5("Et l’unique cordeau des trompettes marinEs"), "c802e1bd9b5f2b0d244bbc982f5082b3");
    }
}