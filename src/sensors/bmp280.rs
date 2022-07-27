use crate::{barometer::Barometer, thermometer::Thermometer};
use embedded_hal::{
    blocking::{i2c, spi},
    digital::v2::OutputPin,
};

pub struct Bmp280<BUS, CS> {
    #[allow(unused)]
    bus: BUS,
    cs: Option<CS>,
}

pub enum Error {
    GpioError,
}

pub struct NoPin;

impl<I2C> Bmp280<I2C, NoPin>
where
    I2C: i2c::WriteRead + i2c::Write,
{
    pub fn new_i2c(bus: I2C) -> Self {
        Self { bus, cs: None }
    }
}

impl<SPI, CS> Bmp280<SPI, CS>
where
    SPI: spi::Transfer<u8>,
    CS: OutputPin,
    <SPI as spi::Transfer<u8>>::Error: core::fmt::Debug,
{
    pub fn new_spi(bus: SPI, cs: CS) -> Self {
        Self { bus, cs: Some(cs) }
    }

    #[allow(unused)]
    fn cs_high(&mut self) -> Result<(), Error> {
        self.cs
            .as_mut()
            .unwrap()
            .set_high()
            .map_err(|_| Error::GpioError)
    }

    #[allow(unused)]
    fn cs_low(&mut self) -> Result<(), Error> {
        self.cs
            .as_mut()
            .unwrap()
            .set_low()
            .map_err(|_| Error::GpioError)
    }
}

impl<BUS, CS> Barometer for Bmp280<BUS, CS> {
    type Error = Error;

    fn pressure_kpa(&mut self) -> Result<f32, Self::Error> {
        todo!()
    }
}

impl<BUS, CS> Thermometer for Bmp280<BUS, CS> {
    type Error = Error;

    fn temperature_celsius(&mut self) -> Result<f32, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
