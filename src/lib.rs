pub struct CRC16<const POLY: u16, const INIT: u16, const XOROUT: u16>(u16);
pub type CRC16XModem = CRC16<0x1021, 0x0000, 0x0000>;
pub type CRC16Genibus = CRC16<0x1021, 0xFFFF, 0xFFFF>;
pub type CRC16CDMA2000 = CRC16<0xC867, 0xFFFF, 0x0000>;

impl<const POLY: u16, const INIT: u16, const XOROUT: u16> CRC16<POLY, INIT, XOROUT> {
    pub const fn new() -> Self {
        Self(INIT)
    }
}

impl<const POLY: u16, const INIT: u16, const XOROUT: u16> CRC for CRC16<POLY, INIT, XOROUT> {
    type Output = u16;

    fn push_bit(&mut self, inc: u8) {
        debug_assert!(inc < 2);

        let xor = [0, POLY][(self.0 >> 15) as usize];
        self.0 <<= 1;
        self.0 |= inc as u16;
        self.0 ^= xor;
    }

    fn output(mut self) -> Self::Output {
        self.write(&[0, 0]);
        self.0 ^ XOROUT
    }

    fn verify(self) -> bool {
        self.0 == 0
    }
}

pub trait CRC {
    type Output;
    fn push_bit(&mut self, inc: u8);
    fn output(self) -> Self::Output;
    fn verify(self) -> bool;

    fn write(&mut self, bytes: &[u8]) {
        for c in bytes {
            let mut c: u8 = *c;
            for _ in 0..8 {
                self.push_bit(c >> 7);
                c <<= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CRC, CRC16CDMA2000, CRC16Genibus, CRC16XModem};

    #[test]
    fn crc16_xmodem() {
        let mut crc = CRC16XModem::new();
        crc.write(b"123456789");
        assert_eq!(crc.output(), 0x31C3);
    }

    #[test]
    fn crc16_genibus() {
        let mut crc = CRC16Genibus::new();
        crc.write(b"123456789");
        assert_eq!(crc.output(), 0xD64E);
    }

    #[test]
    fn crc16_cdma2000() {
        let mut crc = CRC16CDMA2000::new();
        crc.write(b"123456789");
        assert_eq!(crc.output(), 0x4C06);
    }
}
