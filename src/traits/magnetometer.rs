use micromath::vector::F32x3;

pub trait Magnetometer {
    type Error;

    fn magnetic_reading(&mut self) -> Result<F32x3, Self::Error>;
}
