use crate::math::Distance;
use crate::{Massive, Positioned};

const G: f64 = 6.67430e-11;

pub trait Force<T> {
    // Returns the magnitude of the force calculated between
    // two objects of type T.
    fn calculate_between(lhs: T, rhs: T) -> f64;
}

pub struct Gravity {}

impl<'a, T> Force<&'a T> for Gravity
where
    T: Positioned<'a> + Massive,
{
    fn calculate_between(lhs: &'a T, rhs: &'a T) -> f64 {
        let distance = lhs.position().distance(rhs.position());
        G * lhs.mass() * rhs.mass() / distance.powi(2)
    }
}
