use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Scalar<T>(pub T)
where
    T: Copy + Clone + PartialEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector1<T>(pub T)
where
    T: Copy + Clone + PartialEq;

impl<T> Add for Vector1<T>
where
    T: Add<Output = T> + Copy + Clone + PartialEq,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<T> Sub for Vector1<T>
where
    T: Sub<Output = T> + Copy + Clone + PartialEq,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl<T> Mul<Scalar<T>> for Vector1<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialEq,
{
    type Output = Self;

    fn mul(self, scalar: Scalar<T>) -> Self::Output {
        Self(self.0 * scalar.0)
    }
}

impl<T> Mul<Vector1<T>> for Scalar<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialEq,
{
    type Output = Vector1<T>;

    fn mul(self, vector: Vector1<T>) -> Self::Output {
        Vector1(self.0 * vector.0)
    }
}

impl<T> Mul<T> for Vector1<T>
where
    T: Mul<Output = T> + Copy + Clone + PartialEq,
{
    type Output = Vector1<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vector1(self.0 * scalar)
    }
}

impl Mul<Vector1<f32>> for f32 {
    type Output = Vector1<f32>;
    fn mul(self, vector: Vector1<f32>) -> Self::Output {
        Vector1(self * vector.0)
    }
}

impl Mul<Vector1<i32>> for i32 {
    type Output = Vector1<i32>;
    fn mul(self, vector: Vector1<i32>) -> Self::Output {
        Vector1(self * vector.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2(pub f32, pub f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_a_vector1_instance_with_a_float_value_works() {
        Vector1(5.0);
    }

    #[test]
    fn constructing_a_vector1_instance_with_an_integer_value_works() {
        Vector1(-4);
    }

    #[test]
    fn two_equivalent_vector1_instances_are_equal() {
        let first = Vector1(5.0);
        let second = Vector1(5.0);
        assert_eq!(first, second);
    }

    #[test]
    fn two_different_vector1_instances_are_unequal() {
        let second = Vector1(5.0);
        let third = Vector1(6.0);
        assert_ne!(second, third);
    }

    #[test]
    fn adding_two_vector1_instances_works() {
        let first = Vector1(1.0);
        let second = Vector1(2.0);
        assert_eq!(first + second, Vector1(3.0));
    }

    #[test]
    fn subtracting_two_vector1_instances_works() {
        let third = Vector1(3.0);
        let second = Vector1(2.0);
        assert_eq!(third - second, Vector1(1.0));
    }

    #[test]
    fn multiplying_a_scalar_by_a_vector1_works() {
        let first = Vector1(5.0);
        assert_eq!(5.0 * first, Vector1(25.0));
    }

    #[test]
    fn multiplying_a_vector1_by_a_scalar_works() {
        let v = Vector1(5.0);
        assert_eq!(v * 2.0, Vector1(10.0));
    }

    #[test]
    fn multiplying_a_vector1_by_an_integer_scalar_works() {
        let v = Vector1(6);
        assert_eq!(v * 2, Vector1(12));
    }

    #[test]
    fn multiplying_an_integer_scalar_by_a_vector1_works() {
        assert_eq!(3 * Vector1(3), Vector1(9));
    }

    #[test]
    fn two_equivalent_integer_vector1_instances_are_equal() {
        let first_i32 = Vector1(9);
        let second_i32 = Vector1(9);
        assert_eq!(first_i32, second_i32);
    }

    #[test]
    fn two_different_integer_vector1_instances_are_unequal() {
        let first_i32 = Vector1(5);
        let second_i32 = Vector1(-4);
        assert_ne!(first_i32, second_i32);
    }

    #[test]
    fn adding_two_integer_vector1_instances_works() {
        assert_eq!(Vector1(2) + Vector1(3), Vector1(5));
    }

    #[test]
    fn creating_a_vector2_instance_works() {
        Vector2(1.0, 2.0);
    }

    #[test]
    fn two_equivalent_vector2_instances_are_equal() {
        let first_2 = Vector2(1.0, 2.0);
        let second_2 = Vector2(1.0, 2.0);
        assert_eq!(first_2, second_2);
    }

    #[test]
    fn two_different_vector2_instances_are_unequal() {
        let second_2 = Vector2(1.0, 2.0);
        let third_2 = Vector2(3.0, 4.0);
        assert_ne!(second_2, third_2);
    }
}
