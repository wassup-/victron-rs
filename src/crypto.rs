use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherError};
type Aes128Ctr128LE = ctr::Ctr128LE<aes::Aes128>;

pub fn decrypt(data: &[u8], key: [u8; 16], iv: u16) -> Result<Vec<u8>, StreamCipherError> {
    let iv = (iv as u128).to_le_bytes();
    let mut data = pkcs7::pad(data, 16);

    let mut cipher = Aes128Ctr128LE::new(&key.into(), &iv.into());
    if let Err(err) = cipher.try_apply_keystream(&mut data) {
        return Err(err);
    }

    Ok(data)
}

mod pkcs7 {

    pub fn pad(data: &[u8], block_size: usize) -> Vec<u8> {
        let num_pad = block_size - (data.len() % block_size);

        let mut res = data.to_vec();

        if num_pad != block_size {
            res.resize(data.len() + num_pad, num_pad as u8);
        }

        res
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_pkcs7_pad() {
        assert_eq!(pkcs7::pad(&[4; 16], 16), [4; 16]);

        assert_eq!(
            pkcs7::pad(&[1, 2], 16),
            [1, 2, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14]
        );

        assert_eq!(
            pkcs7::pad(&[3; 26], 16),
            [
                3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 6, 6,
                6, 6, 6, 6
            ]
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(
            decrypt(
                &[79, 15, 41, 105, 167, 115, 2, 192, 164, 234, 145, 43, 195, 107, 71, 217],
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF],
                0x1234
            )
            .unwrap(),
            &[102, 111, 111, 98, 97, 114, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10]
        );
    }

    use super::*;
}
