pub mod bias;
pub mod sawtooth;
pub mod sine;
pub mod square;

pub trait PeriodicFunction {
    fn sample(&self, t: f64) -> f64;
}
