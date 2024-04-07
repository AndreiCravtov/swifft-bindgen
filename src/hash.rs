use swifft::{
    hash::{M, INPUT_BLOCK_SIZE, parse_input_block, swifft_hash},
    polynomial::Polynomial
};

pub const IMAGE_PIXELS: usize = 1280 * 720;
pub const BITS_PER_PIXEL: usize = 24;
pub const IMAGE_BITS: usize = IMAGE_PIXELS * BITS_PER_PIXEL;
pub const IMAGE_BYTES: usize = IMAGE_BITS.div_ceil(u8::BITS as usize);
const IMAGE_INPUT_BLOCKS: usize = IMAGE_BYTES.div_ceil(INPUT_BLOCK_SIZE); // 21600
const SECOND_ROUND_DIGEST: usize = IMAGE_INPUT_BLOCKS.div_ceil(M); // 1350
const THIRD_ROUND_DIGEST: usize = SECOND_ROUND_DIGEST.div_ceil(M); // 85
const FOURTH_ROUND_DIGEST: usize = SECOND_ROUND_DIGEST.div_ceil(M); // 6

pub fn swifft_hash_1280_720_24(image_data: Vec<u8>) -> [u16; 64] {
    // make vec_iterator
    assert_eq!(image_data.len(), IMAGE_BYTES);
    let mut vec_iterator = image_data.iter();

    // first round of hashing
    let mut first_round_digest: Vec<Polynomial> = Vec::with_capacity(IMAGE_INPUT_BLOCKS);
    for _ in 0..IMAGE_INPUT_BLOCKS {
        // collect input block
        let mut input_block: [u8; INPUT_BLOCK_SIZE] = [0; INPUT_BLOCK_SIZE];
        for i in 0..INPUT_BLOCK_SIZE {
            input_block[i] = *vec_iterator.next().unwrap();
        }

        // hash and collect digest
        first_round_digest.push(
            swifft_hash(&parse_input_block(&input_block)))
    }

    // make vec_iterator
    assert_eq!(first_round_digest.len(), IMAGE_INPUT_BLOCKS);
    let mut vec_iterator = first_round_digest.iter();

    // second round of hashing
    let mut second_round_digest: Vec<Polynomial> = Vec::with_capacity(SECOND_ROUND_DIGEST);
    for _ in 0..SECOND_ROUND_DIGEST {
        // collect input
        let mut input: [Polynomial; M] = [Polynomial::ZERO; M];
        for i in 0..M {
            match vec_iterator.next() {
                Some(&polynomial) => input[i] = polynomial,
                _ => {}
            }
        }

        // hash and collect digest
        second_round_digest.push(swifft_hash(&input))
    }


    // make vec_iterator
    assert_eq!(second_round_digest.len(), SECOND_ROUND_DIGEST);
    let mut vec_iterator = second_round_digest.iter();

    // third round of hashing
    let mut third_round_digest: Vec<Polynomial> = Vec::with_capacity(THIRD_ROUND_DIGEST);
    for _ in 0..THIRD_ROUND_DIGEST {
        // collect input
        let mut input: [Polynomial; M] = [Polynomial::ZERO; M];
        for i in 0..M {
            match vec_iterator.next() {
                Some(&polynomial) => input[i] = polynomial,
                _ => {}
            }
        }

        // hash and collect digest
        third_round_digest.push(swifft_hash(&input))
    }

    // make vec_iterator
    assert_eq!(third_round_digest.len(), THIRD_ROUND_DIGEST);
    let mut vec_iterator = third_round_digest.iter();

    // fourth round of hashing
    let mut fourth_round_digest: Vec<Polynomial> = Vec::with_capacity(FOURTH_ROUND_DIGEST);
    for _ in 0..FOURTH_ROUND_DIGEST {
        // collect input
        let mut input: [Polynomial; M] = [Polynomial::ZERO; M];
        for i in 0..M {
            match vec_iterator.next() {
                Some(&polynomial) => input[i] = polynomial,
                _ => {}
            }
        }

        // hash and collect digest
        fourth_round_digest.push(swifft_hash(&input))
    }

    // make vec_iterator
    assert_eq!(fourth_round_digest.len(), FOURTH_ROUND_DIGEST);
    let mut vec_iterator = fourth_round_digest.iter();

    // final round of hashing
    let mut input: [Polynomial; M] = [Polynomial::ZERO; M];
    for i in 0..M {
        match vec_iterator.next() {
            Some(&polynomial) => input[i] = polynomial,
            _ => {}
        }
    }
    swifft_hash(&input).coefficients().map(|c| c.value())
}