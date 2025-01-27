use super::{CSIndex, CSxPWMs};

mod spi;

#[test]
fn csindex_to_start_register() {
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(1).to_pwm_start_address()),
        0x001
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(4).to_pwm_start_address()),
        0x037
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(14).to_pwm_start_address()),
        0x0EB
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(15).to_pwm_start_address()),
        0x101
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(16).to_pwm_start_address()),
        0x113
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(17).to_pwm_start_address()),
        0x125
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(28).to_pwm_start_address()),
        0x1EB
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(29).to_pwm_start_address()),
        0x201
    );
    assert_eq!(
        Into::<u16>::into(CSIndex::from_n(33).to_pwm_start_address()),
        0x249
    );
}

#[test]
fn csx_pwms() {
    // Test idx=0
    let mut buf = CSxPWMs::default();
    buf.set_sw_12bit(0, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0b1100_0011, 0x00, 0b0000_1010, 0x00, 0x00, 0x00]
    );

    // Test idx=1
    let mut buf = CSxPWMs::default();
    buf.set_sw_12bit(1, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0b1100_0011, 0b1010_0000, 0x00, 0x00, 0x00]
    );

    // Test idx=2
    let mut buf = CSxPWMs::default();
    buf.set_sw_12bit(2, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0b1100_0011, 0x00, 0b0000_1010]
    );

    // Test idx=3
    let mut buf = CSxPWMs::default();
    buf.set_sw_12bit(3, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0x00, 0b1100_0011, 0b1010_0000]
    );

    // Test idx=2&3
    let mut buf = CSxPWMs::default();
    buf.set_sw_12bit(2, 0b1100_0011_1010);
    buf.set_sw_12bit(3, 0b1100_0011_1010);

    assert_eq!(
        &buf.0[0..6],
        &[0x00, 0x00, 0x00, 0b1100_0011, 0b1100_0011, 0b1010_1010]
    );
}
