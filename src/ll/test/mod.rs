use super::CSxPWMs;

mod spi;

#[test]
fn csx_pwms() {
    // Test idx=0
    let mut buf = CSxPWMs::default();
    buf.set_sw(0, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0b1100_0011, 0x00, 0b0000_1010, 0x00, 0x00, 0x00]
    );

    // Test idx=1
    let mut buf = CSxPWMs::default();
    buf.set_sw(1, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0b1100_0011, 0b1010_0000, 0x00, 0x00, 0x00]
    );

    // Test idx=2
    let mut buf = CSxPWMs::default();
    buf.set_sw(2, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0b1100_0011, 0x00, 0b0000_1010]
    );

    // Test idx=3
    let mut buf = CSxPWMs::default();
    buf.set_sw(3, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0x00, 0b1100_0011, 0b1010_0000]
    );

    // Test idx=2&3
    let mut buf = CSxPWMs::default();
    buf.set_sw(2, 0b1100_0011_1010);
    buf.set_sw(3, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0b1100_0011, 0b1100_0011, 0b1010_1010]
    );
}
