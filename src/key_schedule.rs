use crate::pdc;


static PDC_C1: u128 = 0x173e48cd64013002cdb096a090c4ed7d;
static PDC_C2: u128 = 0x78aaa20eb1e0b84717d0d336b4f620df;

pub fn read_u128_wrap_around(data: &Vec<u8>, index: usize) -> u128 {
    let mut i: u128 = 0;
    let j: usize = data.len();

    for n in 0..16 {
        i += (data[(index + n) as usize % j] as u128) << (n * 8);
    }
    
    return i;
}

pub fn get_round_keys(key: &Vec<u8>) -> Vec<u128> {
    if key.len() % 4 != 0 || key.len() < 16 {
        panic!("Invalid key length; Key must be at least 16 bytes and have a length that is divisible by 4");
    }

    let mut round_keys: Vec<u128> = Vec::new();
    let key_length = key.len();
    let rounds = 4 + (key_length as u32) / 4;

    let mut tmp1: u8 = 0;

    let mut ST0: u128 = 0x8fe8eadc15e1e849e58169d745bb0e5c; // PDC_C1 + PDC_C2
    let mut ST1: u128 = 0x428cebec172c58df4511ad3a38ac3014; // ROTL(PDC_C1, 24) ⊕ ST0
    let mut ST2: u128 = 0xb819d2b227a258f27dc4ad68a97c3506; // ROTR(NOT(ST1), 48) ⊕ ROTL(ST0, 16)
    let mut ST3: u128 = 0x0a08963b82f7f18d00c711d19f92f170; // ROTL(ST1 + NOT(ST2), 32) ⊕ ROTR(ST0, 64)

    for i in 0..rounds {
        let mut state: u128 = PDC_C1;
        let mut tmp2:  u128 = 0;

        if i != 0 {
            state = round_keys[(i - 1) as usize];
        }

        for j in 0..(key_length * 2 + 128) {
            tmp1 += pdc::SBOX[((j as u8 + key[j % key_length]) ^ ((state >> tmp2) as u8)) as usize];
            state = state.rotate_left(tmp2 as u32) ^ tmp1 as u128;
            
            if j % 8 == 0 && i > 4 {
                state += PDC_C2;
                state ^= read_u128_wrap_around(&key, ((state >> 72) as usize) % key_length).rotate_left(16 + ((j as u32 / 8) % 80));
                state = state.rotate_right(9 + (tmp2 as u32 % 7)) ^ round_keys[((state - j as u128) % (i as u128 - 1)) as usize];

                ST0 ^= ST3 +  state.rotate_left  (32 + ((state >> 95) % 94) as u32);
                ST1 += ST2 ^ !state.rotate_left  (48 + ((state >> 74) % 41) as u32);
                ST2 ^= ST1 + !state.rotate_right (64 + ((state >> 32) % 15) as u32);
                ST3 += ST1 ^  state.rotate_right (72 + ((state >> 88) % 33) as u32);

                if j % 48 == 0 {
                    ST0 ^= read_u128_wrap_around(&key, (ST2 ^ state.rotate_left(32)).rotate_left(11 + tmp2 as u32) as usize % key_length)
                        .rotate_left((tmp2 as u32 + 18) % 128);
                    ST3 ^= !(ST0 + PDC_C2);
                    ST1 ^= read_u128_wrap_around(&key, (ST2 ^ state.rotate_left(48)).rotate_left(29 + tmp2 as u32) as usize % key_length)
                        .rotate_left((tmp2 as u32 + 25) % 91); 
                    ST2 ^= !(ST1 + PDC_C1);
                }
            }

            tmp2 = (tmp2 + 8) % 128;
        }

        round_keys.push(state);
    }

    round_keys[0] ^= ST0;
    round_keys[1] ^= ST1;
    round_keys[2] ^= ST2;
    round_keys[3] ^= ST3;

    return round_keys;
}

pub fn reverse_round_keys(round_keys: Vec<u128>) -> Vec<u128> {
    let mut reversed_round_keys: Vec<u128> = Vec::new();

    for i in (0..round_keys.len()).rev() {
        reversed_round_keys.push(round_keys[i]);
    }

    return reversed_round_keys;
}