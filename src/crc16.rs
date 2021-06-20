use crate::CRC;

pub struct CRC16<
    const POLY: u16,
    const INIT: u16,
    const XOROUT: u16,
    const REFIN: bool,
    const REFOUT: bool,
>;
pub type CRC16XModem = CRC16<0x1021, 0x0000, 0x0000, false, false>;
pub type CRC16Genibus = CRC16<0x1021, 0xFFFF, 0xFFFF, false, false>;
pub type CRC16CDMA2000 = CRC16<0xC867, 0xFFFF, 0x0000, false, false>;

impl<
        const POLY: u16,
        const INIT: u16,
        const XOROUT: u16,
        const REFIN: bool,
        const REFOUT: bool,
    > CRC16<POLY, INIT, XOROUT, REFIN, REFOUT>
{
    const fn gen_byte(b: u8) -> u16 {
        let crc = (b as u16) << 8;

        // unrolled `for i in 0..8`
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 15) as usize];

        crc
    }

    const fn init() -> u16 {
        INIT
    }

    const fn next(crc: u16, c: u8) -> u16 {
        let c = if REFIN { c.reverse_bits() } else { c };
        let c = (crc >> 8) as u8 ^ c;
        (crc << 8) ^ Self::gen_byte(c)
    }

    const fn finish(crc: u16) -> u16 {
        let crc = if REFOUT { crc.reverse_bits() } else { crc };
        crc ^ XOROUT
    }

    pub const CHECK: u16 = {
        let crc = Self::init();

        let crc = Self::next(crc, b'0' + 1);
        let crc = Self::next(crc, b'0' + 2);
        let crc = Self::next(crc, b'0' + 3);
        let crc = Self::next(crc, b'0' + 4);
        let crc = Self::next(crc, b'0' + 5);
        let crc = Self::next(crc, b'0' + 6);
        let crc = Self::next(crc, b'0' + 7);
        let crc = Self::next(crc, b'0' + 8);
        let crc = Self::next(crc, b'0' + 9);

        Self::finish(crc)
    };
}

impl<
        const POLY: u16,
        const INIT: u16,
        const XOROUT: u16,
        const REFIN: bool,
        const REFOUT: bool,
    > CRC for CRC16<POLY, INIT, XOROUT, REFIN, REFOUT>
{
    type Output = u16;

    fn calculate(bytes: &[u8]) -> u16 {
        let crc = bytes.iter().map(|&c| c).fold(Self::init(), Self::next);
        Self::finish(crc)
    }
}

#[cfg(test)]
mod tests {
    use super::{CRC16Genibus, CRC16XModem, CRC16CDMA2000};

    #[test]
    fn crc16_xmodem() {
        assert_eq!(CRC16XModem::CHECK, 0x31C3);
    }

    #[test]
    fn crc16_genibus() {
        assert_eq!(CRC16Genibus::CHECK, 0xD64E);
    }

    #[test]
    fn crc16_cdma2000() {
        assert_eq!(CRC16CDMA2000::CHECK, 0x4C06);
    }
}
