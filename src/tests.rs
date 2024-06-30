use crate::{pdc, pdc_cbc};

#[cfg(build = "debug")]
pub fn do_tests() -> bool {
    let mut data: Vec<u8> = vec![56, 244, 225, 134, 239, 236, 9, 133, 57, 37, 199, 121, 109, 63, 179, 210, 33, 160, 5, 64, 25, 166, 116, 255, 237, 72, 112, 26, 77, 234, 122, 252];
    let key: Vec<u8> = vec![225, 54, 45, 98, 189, 161, 240, 148, 54, 140, 107, 92, 23, 164, 247, 27];
    let iv: Vec<u8> = vec![174, 178, 209, 255, 200, 188, 152, 27, 97, 83, 67, 227, 19, 255, 251, 148];

    let data_original = data.clone();
    pdc::encrypt_data(&mut data, &key);
    pdc::decrypt_data(&mut data, &key);

    let data_encrypted_cbc = pdc_cbc::encrypt_data(&data, &key, &iv);

    assert_eq!(data, data_original);
    assert_eq!(data, pdc_cbc::decrypt_data(&data_encrypted_cbc, &key, &iv));

    true
}