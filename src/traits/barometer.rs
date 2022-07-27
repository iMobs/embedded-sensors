pub trait Barometer {
    type Error;

    fn pressure_kpa(&mut self) -> Result<f32, Self::Error>;
}
