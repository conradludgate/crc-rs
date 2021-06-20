use crate::CRC;

pub struct CRC8Impl<
    const POLY: u8,
    const INIT: u8,
    const XOROUT: u8,
    const REFIN: bool,
    const REFOUT: bool,
>;
// pub type CRC8XModem = CRC8<0x1021, 0x0000, 0x0000, false, false>;
// pub type CRC8Genibus = CRC8<0x1021, 0xFFFF, 0xFFFF, false, false>;
// pub type CRC8CDMA2000 = CRC8<0xC867, 0xFFFF, 0x0000, false, false>;

pub type CRC8 = CRC8Impl<0x07, 0x00, 0x00, false, false>;
pub type CRC8CDMA2000 = CRC8Impl<0x9B, 0xFF, 0x00, false, false>;
pub type CRC8DARC = CRC8Impl<0x39, 0x00, 0x00, true, true>;
pub type CRC8DVBS2 = CRC8Impl<0xD5, 0x00, 0x00, false, false>;

// CRC-8/EBU 	1D 	FF 	00 	True 	True
// CRC-8/I-CODE 	1D 	FD 	00 	False 	False
// CRC-8/ITU 	07 	00 	55 	False 	False
// CRC-8/MAXIM 	31 	00 	00 	True 	True
// CRC-8/ROHC 	07 	FF 	00 	True 	True
// CRC-8/WCDMA 	9B 	00 	00 	True 	True

impl<
        const POLY: u8,
        const INIT: u8,
        const XOROUT: u8,
        const REFIN: bool,
        const REFOUT: bool,
    > CRC8Impl<POLY, INIT, XOROUT, REFIN, REFOUT>
{
    const fn gen_byte(b: u8) -> u8 {
        let crc = b;

        // unrolled `for i in 0..8`
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];
        let crc = (crc << 1) ^ [0, POLY][(crc >> 7) as usize];

        crc
    }

    const fn init() -> u8 {
        INIT
    }

    const fn next(crc: u8, c: u8) -> u8 {
        let c = if REFIN { c.reverse_bits() } else { c };
        let c = crc ^ c;
        Self::gen_byte(c)
    }

    const fn finish(crc: u8) -> u8 {
        let crc = if REFOUT { crc.reverse_bits() } else { crc };
        crc ^ XOROUT
    }

    pub const CHECK: u8 = {
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
        const POLY: u8,
        const INIT: u8,
        const XOROUT: u8,
        const REFIN: bool,
        const REFOUT: bool,
    > CRC for CRC8Impl<POLY, INIT, XOROUT, REFIN, REFOUT>
{
    type Output = u8;

    fn calculate(bytes: &[u8]) -> u8 {
        let crc = bytes.iter().map(|&c| c).fold(Self::init(), Self::next);
        Self::finish(crc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        assert_eq!(CRC8::CHECK, 0xF4);
        assert_eq!(CRC8CDMA2000::CHECK, 0xDA);
        assert_eq!(CRC8DARC::CHECK, 0x15);
        assert_eq!(CRC8DVBS2::CHECK, 0xBC)
    }
}
