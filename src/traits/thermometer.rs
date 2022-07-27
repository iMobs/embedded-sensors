pub trait Thermometer {
    type Error;

    fn temperature_celsius(&mut self) -> Result<f32, Self::Error>;

    fn temperature_fahrenheit(&mut self) -> Result<f32, Self::Error> {
        let fahrenheit = (self.temperature_celsius()? * 9.0 / 5.0) + 32.0;
        Ok(fahrenheit)
    }

    fn temperature_kelvin(&mut self) -> Result<f32, Self::Error> {
        let kelvin = self.temperature_celsius()? + 273.15;
        Ok(kelvin)
    }
}
