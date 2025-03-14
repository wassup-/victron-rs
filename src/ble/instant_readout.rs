#[derive(Debug)]
pub enum InstantReadoutContainerError {
    InvalidData,
    InvalidKey,
    DecryptFailed(String),
}

#[derive(Eq, PartialEq, Debug)]
pub struct InstantReadoutContainer {
    pub prefix: u16,
    pub model_id: u16,
    pub readout_type: u8,
    pub iv: u16,
    pub encrypted_data: Vec<u8>,
}

impl InstantReadoutContainer {
    pub fn from_data(mut data: Vec<u8>) -> Result<Self, InstantReadoutContainerError> {
        if data.len() < 8 {
            return Err(InstantReadoutContainerError::InvalidData);
        }

        let prefix = u16::from_le_bytes([data[0], data[1]]);
        let model_id = u16::from_le_bytes([data[2], data[3]]);
        let readout_type = data[4];
        let iv = u16::from_le_bytes([data[5], data[6]]);
        let encrypted_data = data.split_off(7);

        Ok(InstantReadoutContainer {
            prefix,
            model_id,
            readout_type,
            iv,
            encrypted_data,
        })
    }

    pub fn decrypt_data(&self, key: &[u8]) -> Result<Vec<u8>, InstantReadoutContainerError> {
        assert!(!self.encrypted_data.is_empty());

        let key_check = self.encrypted_data[0];
        let data = &self.encrypted_data[1..];

        if key[0] != key_check {
            return Err(InstantReadoutContainerError::InvalidKey);
        }

        let Ok(key) = <[u8; 16]>::try_from(key) else {
            return Err(InstantReadoutContainerError::InvalidKey);
        };

        let data = crypto::decrypt(data, key, self.iv)
            .map_err(|err| InstantReadoutContainerError::DecryptFailed(err.to_string()))?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_instant_readout() {
        assert_eq!(
            InstantReadoutContainer::from_data(
                [
                    0x10, 0x00, 0xC8, 0xA3, 0x04, 0xCC, 0x39, 0xB7, 0x89, 0x28, 0x14, 0xCC, 0x5B,
                    0xEA, 0xCF, 0xB9, 0x09, 0xB7
                ]
                .to_vec()
            )
            .unwrap(),
            InstantReadoutContainer {
                prefix: 16,
                model_id: 41928,
                readout_type: 4,
                iv: 14796,
                encrypted_data: vec![
                    0xB7, 0x89, 0x28, 0x14, 0xCC, 0x5B, 0xEA, 0xCF, 0xB9, 0x09, 0xB7
                ]
            }
        );
    }

    #[test]
    fn test_decrypt_data() {
        let container = InstantReadoutContainer {
            prefix: 16,
            model_id: 41920,
            readout_type: 4,
            iv: 7442,
            encrypted_data: vec![
                0x64, 0xca, 0x8d, 0x44, 0x2b, 0x90, 0xbb, 0xdf, 0x6a, 0x8c, 0xba,
            ],
        };

        let decrypted = container
            .decrypt_data(&[
                0x64, 0xba, 0x49, 0xf1, 0xa8, 0x56, 0x2e, 0x45, 0x19, 0x7a, 0x8e, 0x1f, 0xe5, 0x0d,
                0x76, 0x58,
            ])
            .unwrap();
        println!("decrypted: {:02x?}", decrypted);
        assert_eq!(
            decrypted,
            [
                0x00, 0x00, 0x23, 0x05, 0xff, 0x7f, 0x80, 0x00, 0x00, 0x00, 0xcb, 0xdd, 0x49, 0x4c,
                0xc5, 0xd1
            ]
        );
    }

    use super::*;
}

use crate::crypto;
