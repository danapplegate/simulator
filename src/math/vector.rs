use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector<const N: usize>(#[serde(with = "serde_arrays")] [f32; N]);

impl<const N: usize> From<[f32; N]> for Vector<N> {
    fn from(components: [f32; N]) -> Self {
        Self(components)
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self([0.0; N])
    }
}

impl<const N: usize> Index<usize> for Vector<N> {
    type Output = f32;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<const N: usize> IndexMut<usize> for Vector<N> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
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
    pub fn new(x: f32) -> Self {
        Self([x])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }
}

impl Vector<2> {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }
}

impl Vector<3> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn cross(&self, rhs: &Vector<3>) -> Vector<3> {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();
        Vector::<3>::new(x, y, z)
    }

    /// Calculates the normalized vector that is normal to the plane formed
    /// by the three position vectors. This is a right-handed operation,
    /// with the first or A vector computed as the difference between v2
    /// and v1, and the second or B vector computed as the difference between
    /// v3 and v1.
    pub fn normal(v1: &Self, v2: &Self, v3: &Self) -> Self {
        let u = v2 - v1;
        let v = v3 - v1;
        let cross = u.cross(&v);
        cross.normalize()
    }
}

pub type Vector1 = Vector<1>;
pub type Vector2 = Vector<2>;
pub type Vector3 = Vector<3>;

pub trait Distance {
    type Output: Distance;

    fn distance(&self, other: &Self) -> f32;
    fn direction(&self, to: &Self) -> Self::Output;
    fn normalize(&self) -> Self::Output;
    fn magnitude(&self) -> f32;
}

impl<const N: usize> Distance for Vector<N> {
    type Output = Vector<N>;

    fn distance(&self, other: &Self) -> f32 {
        (self - other).magnitude()
    }

    fn direction(&self, to: &Self) -> Self::Output {
        (to - self).normalize()
    }

    fn normalize(&self) -> Self::Output {
        self / self.magnitude()
    }

    fn magnitude(&self) -> f32 {
        let mut sum_of_squares = 0_f32;
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

impl<const N: usize> Div<f32> for &Vector<N> {
    type Output = Vector<N>;
    fn div(self, other: f32) -> Self::Output {
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

impl<const N: usize> Mul<&Vector<N>> for f32 {
    type Output = Vector<N>;
    fn mul(self, other: &Vector<N>) -> Self::Output {
        let mut components = [0.0; N];
        for i in 0..N {
            components[i] = self * other.0[i];
        }
        Vector::<N>(components)
    }
}

impl<const N: usize> Vector<N> {
    pub fn dot(&self, rhs: &Self) -> f32 {
        let mut sum = 0.0;
        for i in 0..N {
            sum += self.0[i] * rhs.0[i];
        }
        sum
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
    fn normalization_of_a_vector2_instance_works() {
        let first = Vector2::new(3.0, 4.0);
        assert_eq!(first.normalize(), Vector2::new(0.6, 0.8));
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

    #[test]
    fn taking_the_cross_product_of_two_axes_gives_the_third() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.cross(&v2), Vector3::new(0.0, 0.0, 1.0));
        assert_eq!(v2.cross(&v1), Vector3::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn dot_product_of_two_orthogonal_2d_vectors_is_zero() {
        let v1 = Vector2::new(2.0, 1.0);
        let v2 = Vector2::new(-1.0, 2.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);
    }

    #[test]
    fn dot_product_of_two_orthogonal_3d_vectors_is_zero() {
        let v1 = Vector3::new(1.0, 0.0, 1.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        assert_eq!(v1.dot(&v2), 0.0);
        assert_eq!(v2.dot(&v1), 0.0);
    }

    #[test]
    fn dot_product_of_two_vectors_at_sixty_degrees() {
        let v1 = Vector2::new(1.0, 0.0);
        let v2 = Vector2::new(0.5, 0.8660254038);
        assert_eq!(v1.dot(&v2), 0.5);
    }

    #[test]
    fn calculating_the_normal_of_a_plane_works() {
        // The back face of a triangular prism
        let v1 = Vector3::new(0.5, 0.5, -0.5);
        let v2 = Vector3::new(0.5, -0.5, -0.5);
        let v3 = Vector3::new(-0.5, -0.5, -0.5);
        assert_eq!(Vector3::normal(&v1, &v2, &v3), Vector3::new(0.0, 0.0, -1.0));
    }
}
