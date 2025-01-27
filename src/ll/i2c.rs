use device_driver::AsyncCommandInterface;
use embedded_hal_async::i2c::I2c;

const MAX_WRITE_SIZE: usize = 2;

use crate::ll::{DeviceError, RegisterAddress};

use crate::ll::Passphrase;

pub struct DeviceInterface<I2C: I2c> {
    i2c: I2C,
    address: u8,
    page: u8,
}

impl<I2C: I2c> DeviceInterface<I2C> {
    /// Construct a new instance of the device.
    ///
    /// I2C max frequency 1MHz.
    pub const fn new(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            page: 0xff, // use some bogus page
        }
    }

    async fn ensure_page(&mut self, page: u8) -> Result<(), DeviceError<I2C::Error>> {
        if self.page == page {
            return Ok(());
        }

        self.dispatch_command(0xff, 8, &[Passphrase::Unlocked.into()], 0, &mut [])
            .await?;
        self.dispatch_command(0xfe, 8, &[page], 0, &mut []).await
    }
}

impl<I2C: I2c> device_driver::AsyncRegisterInterface for DeviceInterface<I2C> {
    type Error = DeviceError<I2C::Error>;

    type AddressType = u16;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        let address: RegisterAddress = address.into();
        self.ensure_page(address.page()).await?;

        let mut vec = heapless::Vec::<u8, MAX_WRITE_SIZE>::new();
        vec.push(address.register())
            .map_err(|_| DeviceError::BufferTooSmall)?;
        vec.extend_from_slice(data)
            .map_err(|_| DeviceError::BufferTooSmall)?;
        self.i2c.write(self.address, &vec).await?;
        Ok(())
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        let address: RegisterAddress = address.into();
        self.ensure_page(address.page()).await?;

        Ok(self
            .i2c
            .write_read(self.address, &[address.register()], data)
            .await?)
    }
}

impl<I2C: I2c> device_driver::AsyncCommandInterface for DeviceInterface<I2C> {
    type Error = DeviceError<I2C::Error>;

    type AddressType = u8;

    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        _size_bits_in: u32,
        input: &[u8],
        _size_bits_out: u32,
        _output: &mut [u8],
    ) -> Result<(), Self::Error> {
        let mut vec = heapless::Vec::<u8, MAX_WRITE_SIZE>::new();
        vec.push(address).map_err(|_| DeviceError::BufferTooSmall)?;
        vec.extend_from_slice(input)
            .map_err(|_| DeviceError::BufferTooSmall)?;
        self.i2c.write(self.address, &vec).await?;
        Ok(())
    }
}
