use crate::barometer::Barometer;
use micromath::F32Ext;

/// # TODO
/// Is the sea level KPA necessary for the generic trait or is that only useful
/// for when the trait is implemented for a barometer?
pub trait Altimeter {
    type Error;

    fn altitude_meters(&mut self, sea_level_kpa: f32) -> Result<f32, Self::Error>;

    fn altitude_kilometers(&mut self, sea_level_kpa: f32) -> Result<f32, Self::Error> {
        let kilometers = self.altitude_meters(sea_level_kpa)? / 1000.0;
        Ok(kilometers)
    }

    fn altitude_feet(&mut self, sea_level_kpa: f32) -> Result<f32, Self::Error> {
        let feet = self.altitude_meters(sea_level_kpa)? * 3.281;
        Ok(feet)
    }

    fn altitude_miles(&mut self, sea_level_kpa: f32) -> Result<f32, Self::Error> {
        let miles = self.altitude_feet(sea_level_kpa)? * 5280.0;
        Ok(miles)
    }
}

impl<T> Altimeter for T
where
    T: Barometer,
{
    type Error = <Self as Barometer>::Error;

    fn altitude_meters(&mut self, sea_level_kpa: f32) -> Result<f32, Self::Error> {
        let pressure = self.pressure_kpa()?;
        let sea_level_kpa = sea_level_kpa * 1000.0;

        let altitude = 44330.0 * (1.0 - (pressure / sea_level_kpa).powf(0.1903));
        Ok(altitude)
    }
}
