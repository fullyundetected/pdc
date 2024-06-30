pub fn bytes_to_u64(bytes: &[u8]) -> u64 {
    if bytes.len() != 16 {
        return 0;
    }

    let mut buf = [0u8; 8];
    buf[0]  = bytes[0];
    buf[1]  = bytes[1];
    buf[2]  = bytes[2];
    buf[3]  = bytes[3];
    buf[4]  = bytes[4];
    buf[5]  = bytes[5];
    buf[6]  = bytes[6];
    buf[7]  = bytes[7];

    return u64::from_be_bytes(buf);
}

pub fn pad_data(data: &mut Vec<u8>) -> Vec<u8> {
    let mut padded_data: Vec<u8> = Vec::new();
    padded_data.append(&mut (data.len() as u64).to_be_bytes().to_vec());
    padded_data.append(data);
    
    let pad = 16 - padded_data.len() % 16;
    if pad != 16 {
        for i in 0..pad {
            padded_data.push(i as u8);
        }
    }

    return padded_data;
}

pub fn unpad_data(data: &mut Vec<u8>) -> Vec<u8> {
    let unpadded_length = bytes_to_u64(&data[0..8]) as usize;
    return data[8..unpadded_length + 8].to_vec();
}