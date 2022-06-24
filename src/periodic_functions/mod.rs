pub mod sine;

pub trait PeriodicFunction {
    fn sample(&self, t: f32) -> f32;
}