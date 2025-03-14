pub mod instant_readout;

pub const VICTRON_MANUFACTURER_ID: u16 = 0x02E1;

#[derive(Debug)]
pub enum AdvertisementDataError {
    NotEnoughData,
}

pub trait FromAdvertisementData: Sized {
    fn from_advertisement_data(data: &[u8]) -> Result<Self, AdvertisementDataError>;
}
