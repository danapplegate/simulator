use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::{Add, Div, Index, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector<const N: usize>(#[serde(with = "serde_arrays")] [f64; N]);

impl<const N: usize> From<[f64; N]> for Vector<N> {
    fn from(components: [f64; N]) -> Self {
        Self(components)
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<const N: usize> Sum for Vector<N> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vector::default(), |a, v| &a + &v)
    }
}

impl Vector<1> {
    pub fn new(x: f64) -> Self {
        Self([x])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
}

impl Vector<2> {
    pub fn new(x: f64, y: f64) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }
}

impl Vector<3> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

pub type Vector1 = Vector<1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;

pub trait Distance {
    type Output: Distance;

    fn distance(&self, other: &Self) -> f64;
    fn direction(&self, to: &Self) -> Self::Output;
    fn unit(&self) -> Self::Output;
    fn magnitude(&self) -> f64;
}

impl<const N: usize> Distance for Vector<N> {
    type Output = Vector<N>;

    fn distance(&self, other: &Self) -> f64 {
        (self - other).magnitude()
    }

    fn direction(&self, to: &Self) -> Self::Output {
        (to - self).unit()
    }

    fn unit(&self) -> Self::Output {
        self / self.magnitude()
    }

    fn magnitude(&self) -> f64 {
        let mut sum_of_squares = 0_f64;
        for i in 0..N {
            sum_of_squares += self.0[i].powi(2);
        }
        sum_of_squares.sqrt()
    }
}

impl<const N: usize> Sub<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;
    fn sub(self, other: &Vector<N>) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self.0[i] - other.0[i];
        }
        Vector::<N>(components)
    }
}

impl<const N: usize> Div<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: &Vector<N>) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self.0[i] / other.0[i];
        }
        Vector::<N>(components)
    }
}

impl<const N: usize> Div<f64> for &Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: f64) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self.0[i] / other;
        }
        Vector::<N>(components)
    }
}

impl<const N: usize> Add<&Vector<N>> for &Vector<N> {
    type Output = Vector<N>;
    fn add(self, other: &Vector<N>) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self.0[i] + other.0[i];
        }
        Vector::<N>(components)
    }
}

impl<const N: usize> Mul<&Vector<N>> for f64 {
    type Output = Vector<N>;
    fn mul(self, other: &Vector<N>) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self * other.0[i];
        }
        Vector::<N>(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_two_vector1_instances_works() {
        let first = Vector1::new(5.0);
        let second = Vector1::new(2.0);
        assert_eq!(first.distance(&second), 3.0);
        assert_eq!(second.distance(&first), 3.0);
    }

    #[test]
    fn subtracting_two_vector1_instances_works() {
        let first = Vector1::new(3.0);
        let second = Vector1::new(4.0);
        assert_eq!(&first - &second, Vector1::new(-1.0));
        assert_eq!(&second - &first, Vector1::new(1.0));
    }

    #[test]
    fn magnitude_of_a_vector1_instance_works() {
        let first = Vector1::new(-7.0);
        assert_eq!(first.magnitude(), 7.0);
    }

    #[test]
    fn direction_between_two_vector1_instances_works() {
        let first = Vector1::new(1.0);
        let second = Vector1::new(-2.0);
        assert_eq!(first.direction(&second), Vector1::new(-1.0));
    }

    #[test]
    fn distance_between_two_vector2_instances_works() {
        let first = Vector2::new(3.0, 3.0);
        let second = Vector2::new(6.0, 7.0);
        assert_eq!(first.distance(&second), 5.0);
        assert_eq!(second.distance(&first), 5.0);
    }

    #[test]
    fn adding_two_vector2_instances_works() {
        let first = Vector2::new(1.0, 2.0);
        let second = Vector2::new(3.0, 4.0);
        assert_eq!(&first + &second, Vector2::new(4.0, 6.0));
        assert_eq!(&second + &first, Vector2::new(4.0, 6.0));
    }

    #[test]
    fn subtracting_two_vector2_instances_works() {
        let first = Vector2::new(1.0, 2.0);
        let second = Vector2::new(3.0, 4.0);
        assert_eq!(&first - &second, Vector2::new(-2.0, -2.0));
        assert_eq!(&second - &first, Vector2::new(2.0, 2.0));
    }

    #[test]
    fn dividing_a_vector2_instance_by_a_scalar_works() {
        let first = Vector2::new(2.0, 5.0);
        assert_eq!(&first / 2.0, Vector2::new(1.0, 2.5));
    }

    #[test]
    fn multiplying_a_scalar_by_a_vector2_works() {
        let first = Vector2::new(3.0, 4.0);
        assert_eq!(2.0 * &first, Vector2::new(6.0, 8.0));
    }

    #[test]
    fn unit_of_a_vector2_instance_works() {
        let first = Vector2::new(3.0, 4.0);
        assert_eq!(first.unit(), Vector2::new(0.6, 0.8));
    }

    #[test]
    fn distance_between_two_vector3_instances_works() {
        let first = Vector3::new(0.0, 0.0, 0.0);
        let second = Vector3::new(2.0, 3.0, 6.0);
        assert_eq!(first.distance(&second), 7.0);
        assert_eq!(second.distance(&first), 7.0);
    }

    #[test]
    fn magnitude_of_a_vector3_instance_works() {
        let first = Vector3::new(4.0, 4.0, 7.0);
        assert_eq!(first.magnitude(), 9.0);
    }

    #[test]
    fn subtracting_two_vector3_instances_works() {
        let first = Vector3::new(5.0, 3.0, -2.0);
        let second = Vector3::new(1.0, -2.0, 4.0);
        assert_eq!(&first - &second, Vector3::new(4.0, 5.0, -6.0));
        assert_eq!(&second - &first, Vector3::new(-4.0, -5.0, 6.0));
    }

    #[test]
    fn creating_a_vector_from_an_array_works() {
        let v = Vector::from([1.0, 2.0, 3.0]);
        assert_eq!(v, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn indexing_a_vector_works() {
        let v = Vector::from([1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(v[3], 4.0);
    }
}
