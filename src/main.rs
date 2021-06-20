use crc::{CRC, CRC16XModem};

fn main() {
    let mut crc = CRC16XModem::new();
    crc.write(b"123456789");
    assert_eq!(crc.output(), [0x31, 0xC3]);
}
