#![feature(iter_array_chunks)]
mod hash;

pub use wasm_bindgen_rayon::init_thread_pool;
use ff::PrimeField;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array};
use swifft::polynomial::Polynomial;
use swifft::z257::Z257;

use crate::hash::IMAGE_BYTES;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Number of bits needed to fully represent a `SWIFFT` digest, with 64 field elements and 9 bits per element
pub const SWIFFT_DIGEST_BITS: usize = Z257::NUM_BITS as usize * Polynomial::N;

/// Takes in a byte-array input of length `2764800` - representing the bytes of the image,
/// computes a hash of `64` field elements (`9` bits each), and packs those bits into a `72`-byte little endian array
#[wasm_bindgen]
pub fn swifft_hash_1280_720_24(image_data: Uint8Array) -> Uint8Array {
    // compute digest
    assert_eq!(image_data.length() as usize, IMAGE_BYTES);
    let digest = hash::swifft_hash_1280_720_24(image_data.to_vec());
    
    // make bit string with reverse-bit order (LSB first), by extracting the 9 bits from each element
    // (which is initially in MSB order), reversing bit-order, and appending to the bit-string
    let mut bit_string_576: String = String::with_capacity(576);
    for element in digest {
        let bits = format!("{element:b}");
        let bits_9_le: String = format!("{:0>9}", bits).chars().rev().collect();
        bit_string_576.push_str(bits_9_le.as_str());
    }
    assert_eq!(bit_string_576.len(), 576);

    // convert bitstring into little-endian hex string, by reversing bit order (making it big endian)
    // converting each 4-bit nibble into a hex-digit, spliting every 2 hex-digits to create a list of bytes,
    // and reversing the order of those bytes - to achieve little endianness.
    bit_string_576 = bit_string_576.chars().rev().collect(); // MSB first
    let be_hex_string: String = bit_string_576.chars().array_chunks::<4>()
        .map(|nibble| {
            let nibble = nibble.iter().fold(String::new(), |mut acc, next| {
                acc.push(*next);
                acc
            });
            let hex_digit = String::from(nibble_bin_to_hex(nibble.as_str()));
            hex_digit
        }).collect();
    assert_eq!(be_hex_string.len(), 144);
    let mut le_bytes_72: Vec<u8> = Vec::with_capacity(72);
    be_hex_string.chars().array_chunks::<2>()
        .for_each(|byte| {
            // make byte hex
            let mut byte_str = String::new();
            byte_str.push(byte[0]);
            byte_str.push(byte[1]);

            // make byte
            let byte = u8::from_str_radix(byte_str.as_str(), 16).unwrap();

            // insert byte at start - to reversed order
            le_bytes_72.insert(0, byte)
        });
    assert_eq!(le_bytes_72.len(), 72);

    // make return array
    let digest_return = Uint8Array::new_with_length(72);
    Uint8Array::copy_from(&digest_return, &le_bytes_72);
    digest_return
}

fn nibble_bin_to_hex(bin: &str) -> &str {
    match bin {
        "0000" => "0",
        "0001" => "1",
        "0010" => "2",
        "0011" => "3",
        "0100" => "4",
        "0101" => "5",
        "0110" => "6",
        "0111" => "7",
        "1000" => "8",
        "1001" => "9",
        "1010" => "a",
        "1011" => "b",
        "1100" => "c",
        "1101" => "d",
        "1110" => "e",
        "1111" => "f",
        _ => panic!("Expected a binary nibble string, found `{bin}` instead")
    }
}

