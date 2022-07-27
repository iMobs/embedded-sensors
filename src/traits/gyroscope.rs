use micromath::vector::F32x3;

pub trait Gyroscope {
    type Error;

    fn angular_rate_reading(&mut self) -> Result<F32x3, Self::Error>;
}
