use embedded_hal_mock::eh1::spi::{Mock, Transaction};

use crate::ll::{self, CSxPWMs};

fn regw(page: u8, register: u8, values: &[u8]) -> Vec<Transaction<u8>> {
    vec![
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0b1011_0000 | page, register]),
        Transaction::write_vec(values.to_vec()),
        Transaction::transaction_end(),
    ]
}

fn regr(page: u8, register: u8, values: &[u8]) -> Vec<Transaction<u8>> {
    vec![
        Transaction::transaction_start(),
        Transaction::write_vec(vec![0b0011_0000 | page, register]),
        Transaction::read_vec(values.to_vec()),
        Transaction::transaction_end(),
    ]
}

#[async_std::test]
async fn example_general() {
    let expectations = [
        regw(2, 0xDF, &[0xA5]),
        regw(2, 0xA6, &[0b10 << 6 | 12]),
        regr(2, 0xA7, &(1u64 << 22).to_le_bytes()[0..5]),
        regw(2, 0xA6, &[0x00]),
        regw(0, 0xFD, &[0x00]),
    ];

    let mut spi = Mock::new(expectations.iter().flatten());

    let mut ll = ll::Device::new(ll::spi::DeviceInterface::new(&mut spi));

    ll.reset().write_async(|_| ()).await.unwrap();

    ll.open_short()
        .write_async(|w| {
            w.set_sw(12);
            w.set_mode(ll::OpenShortMode::Open);
        })
        .await
        .unwrap();

    assert_eq!(
        ll.open_short_lines().read_async().await.unwrap().sw(),
        1 << 22
    );

    ll.open_short().write_async(|_| ()).await.unwrap();

    ll.pwm_update()
        .dispatch_async(|w| {
            w.set_value(0x00);
        })
        .await
        .unwrap();

    spi.done();
}

#[async_std::test]
async fn example_pwm() {
    let expectations = [
        regw(
            0,
            0x13,
            &[
                0x00, 0x00, 0x00, 0xBA, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        ),
        regw(
            1,
            0xD9,
            &[
                0x00, 0x00, 0x00, 0xBA, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        ),
        regw(0, 0xFD, &[0x00]),
    ];

    let mut spi = Mock::new(expectations.iter().flatten());

    let mut ll = ll::Device::new(ll::spi::DeviceInterface::new(&mut spi));

    let mut buf = CSxPWMs::default();
    buf.set_sw(2, 0b1011_1010_1000);
    ll.pwm_cs(ll::CSIndex::from_n(2), &buf).await.unwrap();
    ll.pwm_cs(ll::CSIndex::from_n(27), &buf).await.unwrap();

    ll.pwm_update()
        .dispatch_async(|w| {
            w.set_value(0x00);
        })
        .await
        .unwrap();

    spi.done();
}
