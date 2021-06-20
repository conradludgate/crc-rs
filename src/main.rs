use crc::{CRC, CRC16XModem};

fn main() {
    assert_eq!(CRC16XModem::calculate(b"123456789"), 0x31C3);
}
