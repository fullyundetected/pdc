use crate::{key_schedule, pdc::{decrypt_block, encrypt_block}};

pub fn encrypt_data(data: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    if data.len() % 16 != 0 {
        panic!("Data length must be a multiple of 16; You must pad your data before encrypting it")
    }
    if iv.len() != 16 {
        panic!("IV must be 16 bytes");
    }

    let mut ciphertext: Vec<u8> = Vec::new();
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let round_keys = key_schedule::get_round_keys(key);

    for i in (0..data.len()).step_by(16) {
        let mut block = data[i..i+16].to_vec();
        if i == 0 {
            for j in 0..16 {
                block[j] ^= iv[j];
            }
        } else {
            let previous_block = &blocks[i / 16 - 1];
            for j in 0..16 {
                block[j] ^= previous_block[j];
            }
        }
        encrypt_block(&mut block, &round_keys);
        blocks.push(block);
    }

    for mut block in blocks {
        ciphertext.append(&mut block);
    }

    return ciphertext;
}

pub fn decrypt_data(data: &Vec<u8>, key: &Vec<u8>, iv: &Vec<u8>) -> Vec<u8> {
    if data.len() % 16 != 0 {
        panic!("Data length must be a multiple of 16; You must pad your data before encrypting it")
    }
    if iv.len() != 16 {
        panic!("IV must be 16 bytes");
    }

    let mut ciphertext: Vec<u8> = Vec::new();
    let mut blocks: Vec<Vec<u8>> = Vec::new();
    let reversed_round_keys = key_schedule::reverse_round_keys(key_schedule::get_round_keys(key));

    for i in (0..data.len()).step_by(16).rev() {
        let mut block = data[i..i+16].to_vec();
        decrypt_block(&mut block, &reversed_round_keys);
        if i / 16 == 0 {
            for j in 0..16 {
                block[j] ^= iv[j];
            }
        } else {
            let previous_block = &data[i-16..i];
            for j in 0..16 {
                block[j] ^= previous_block[j];
            }
        }
        blocks.push(block);
    }
    for i in (0..blocks.len()).rev() {
        ciphertext.append(&mut blocks[i]);
    }

    return ciphertext;
}