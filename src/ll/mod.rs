use device_driver::AsyncRegisterInterface;

#[cfg(test)]
mod test;

pub mod i2c;
pub mod spi;

device_driver::create_device!(
    device_name: Device,
    manifest: "src/ll/ll.yaml"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum DeviceError<T> {
    Interface(T),
    BufferTooSmall,
}

impl<T> From<T> for DeviceError<T> {
    fn from(value: T) -> Self {
        DeviceError::Interface(value)
    }
}

#[derive(PartialEq, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
struct RegisterAddress(u16);

impl RegisterAddress {
    pub const fn new(page: u8, register: u8) -> Self {
        Self(((page as u16) << 8) | (register as u16))
    }

    pub const fn register(&self) -> u8 {
        (self.0 & 0xff) as u8
    }

    pub const fn page(&self) -> u8 {
        (self.0 >> 8) as u8
    }
}

impl From<u16> for RegisterAddress {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Into<u16> for RegisterAddress {
    fn into(self) -> u16 {
        self.0
    }
}

pub struct CSIndex {
    i: u8,
}

impl CSIndex {
    pub const fn from_n(n: u8) -> Self {
        Self { i: n - 1 }
    }

    pub const fn from_i(i: u8) -> Self {
        Self { i }
    }

    pub const fn i(&self) -> u8 {
        self.i
    }

    pub const fn n(&self) -> u8 {
        self.i + 1
    }

    const fn to_pwm_start_address(&self) -> RegisterAddress {
        let page = self.i() / 14;
        let register = (self.i() % 14) * 18 + 1;

        assert!(page <= 2);

        RegisterAddress::new(page, register)
    }
}

#[derive(Default)]
pub struct CSxPWMs([u8; 18]);

impl CSxPWMs {
    pub const fn set_sw(&mut self, sw_i: u8, x: u16) {
        let l = (x & 0x0f) as u8;
        let h = ((x >> 4) & 0xff) as u8;

        let group = (sw_i / 2) as usize;
        let member = (sw_i % 2) as usize;
        let start = group * 3;

        self.0[start + member] = h;

        if member == 0 {
            self.0[start + 2] = (self.0[start + 2] & 0xf0) | l;
        } else {
            self.0[start + 2] = (self.0[start + 2] & 0x0f) | (l << 4);
        }
    }
}

#[allow(private_bounds)]
impl<I: AsyncRegisterInterface> Device<I>
where
    RegisterAddress: Into<I::AddressType>,
{
    pub async fn pwm_cs(&mut self, cs_i: CSIndex, pwms: &CSxPWMs) -> Result<(), I::Error> {
        let address = cs_i.to_pwm_start_address().into();
        self.interface.write_register(address, 144, &pwms.0).await
    }
}
