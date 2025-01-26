use embedded_hal::spi::Operation;
use embedded_hal_async::spi::SpiDevice;

use crate::ll::{DeviceError, RegisterAddress};

pub struct DeviceInterface<SPI: SpiDevice> {
    spi: SPI,
}

impl<SPI: SpiDevice> DeviceInterface<SPI> {
    /// Construct a new instance of the device.
    ///
    /// SPI max frequency 12MHz.
    pub const fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

impl<SPI: SpiDevice> device_driver::AsyncRegisterInterface for DeviceInterface<SPI> {
    type Error = DeviceError<SPI::Error>;

    type AddressType = u16;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let address: RegisterAddress = address.into();
        let preamble = [0b1011_0000 | address.page(), address.register()];
        let mut operations = [Operation::Write(&preamble), Operation::Write(data)];
        self.spi.transaction(&mut operations).await?;
        Ok(())
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let address: RegisterAddress = address.into();
        let preamble = [0b0011_0000 | address.page(), address.register()];
        let mut operations = [Operation::Write(&preamble), Operation::Read(data)];
        self.spi.transaction(&mut operations).await?;
        Ok(())
    }
}

impl<SPI: SpiDevice> device_driver::AsyncCommandInterface for DeviceInterface<SPI> {
    type Error = DeviceError<SPI::Error>;

    type AddressType = u8;

    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        _size_bits_in: u32,
        input: &[u8],
        _size_bits_out: u32,
        _output: &mut [u8],
    ) -> Result<(), Self::Error> {
        let address = [0b1011_0000, address];
        let mut operations = [Operation::Write(&address), Operation::Write(input)];
        self.spi.transaction(&mut operations).await?;
        Ok(())
    }
}
