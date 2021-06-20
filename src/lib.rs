mod crc8;
mod crc16;

pub use crc8::*;
pub use crc16::*;

pub trait CRC {
    type Output;
    fn calculate(bytes: &[u8]) -> Self::Output;
}
