#[derive(PartialEq, Debug)]
pub struct DcDcConverterData {
    pub device_state: u8,
    pub charger_error: u8,
    pub input_voltage: u16,
    pub output_voltage: i16,
    pub off_reason: u32,
}

impl FromAdvertisementData for DcDcConverterData {
    fn from_advertisement_data(data: &[u8]) -> Result<Self, AdvertisementDataError> {
        let mut rdr = BitReader::new(data);

        let device_state = rdr
            .read_u8(8)
            .map_err(|_| AdvertisementDataError::NotEnoughData)?;
        let charger_error = rdr
            .read_u8(8)
            .map_err(|_| AdvertisementDataError::NotEnoughData)?;
        let input_voltage = rdr
            .read_u16(16)
            .map_err(|_| AdvertisementDataError::NotEnoughData)?;
        let output_voltage = rdr
            .read_i16(16)
            .map_err(|_| AdvertisementDataError::NotEnoughData)?;
        let off_reason = rdr
            .read_u32(32)
            .map_err(|_| AdvertisementDataError::NotEnoughData)?;

        Ok(DcDcConverterData {
            device_state,
            charger_error,
            input_voltage,
            output_voltage,
            off_reason,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dc_dc_converter_data_from_advertisement_data() {
        let data = DcDcConverterData::from_advertisement_data(&[
            0x01, 0x02, 0x23, 0x05, 0xff, 0x7f, 0x80, 0x00, 0x00, 0x00, 0xcb, 0xdd, 0x49, 0x4c,
            0xc5, 0xd1,
        ])
        .unwrap();

        assert_eq!(
            data,
            DcDcConverterData {
                device_state: 1,
                charger_error: 2,
                input_voltage: 1315,
                output_voltage: 0x7FFF,
                off_reason: 0x00000080
            }
        );
    }

    use super::*;
}

use crate::{
    ble::{AdvertisementDataError, FromAdvertisementData},
    util::BitReader,
};
