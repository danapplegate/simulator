use std::ops::{Add, Sub};

#[derive(Default, Debug, PartialEq)]
pub struct Vector1 {
    pub x: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub trait Distance {
    fn distance(self, other: Self) -> f64;
    // fn direction(&self, from: Self) -> Self;
    // fn unit(&self) -> Self;
    fn magnitude(self) -> f64;
}

impl Distance for &Vector1 {
    fn distance(self, other: &Vector1) -> f64 {
        (self - other).magnitude()
    }

    fn magnitude(self) -> f64 {
        self.x.abs()
    }
}

impl Sub<&Vector1> for &Vector1 {
    type Output = Vector1;
    fn sub(self, other: &Vector1) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
        }
    }
}

impl Distance for &Vector2 {
    fn distance(self, other: &Vector2) -> f64 {
        (self - other).magnitude()
    }

    fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, other: &Vector2) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, other: &Vector2) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Distance for &Vector3 {
    fn distance(self, other: &Vector3) -> f64 {
        (self - other).magnitude()
    }

    fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, other: &Vector3) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_between_two_vector1_instances_works() {
        let first = Vector1 { x: 5.0 };
        let second = Vector1 { x: 2.0 };
        assert_eq!(first.distance(&second), 3.0);
        assert_eq!(second.distance(&first), 3.0);
    }

    #[test]
    fn subtracting_two_vector1_instances_works() {
        let first = Vector1 { x: 3.0 };
        let second = Vector1 { x: 4.0 };
        assert_eq!(&first - &second, Vector1 { x: -1.0 });
        assert_eq!(&second - &first, Vector1 { x: 1.0 });
    }

    #[test]
    fn magnitude_of_a_vector1_instance_works() {
        let first = Vector1 { x: -7.0 };
        assert_eq!(first.magnitude(), 7.0);
    }

    #[test]
    fn distance_between_two_vector2_instances_works() {
        let first = Vector2 { x: 3.0, y: 3.0 };
        let second = Vector2 { x: 6.0, y: 7.0 };
        assert_eq!(first.distance(&second), 5.0);
        assert_eq!(second.distance(&first), 5.0);
    }

    #[test]
    fn adding_two_vector2_instances_works() {
        let first = Vector2 { x: 1.0, y: 2.0 };
        let second = Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(&first + &second, Vector2 { x: 4.0, y: 6.0 });
        assert_eq!(&second + &first, Vector2 { x: 4.0, y: 6.0 });
    }

    #[test]
    fn subtracting_two_vector2_instances_works() {
        let first = Vector2 { x: 1.0, y: 2.0 };
        let second = Vector2 { x: 3.0, y: 4.0 };
        assert_eq!(&first - &second, Vector2 { x: -2.0, y: -2.0 });
        assert_eq!(&second - &first, Vector2 { x: 2.0, y: 2.0 });
    }

    #[test]
    fn distance_between_two_vector3_instances_works() {
        let first = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let second = Vector3 {
            x: 2.0,
            y: 3.0,
            z: 6.0,
        };
        assert_eq!(first.distance(&second), 7.0);
        assert_eq!(second.distance(&first), 7.0);
    }

    #[test]
    fn magnitude_of_a_vector3_instance_works() {
        let first = Vector3 {
            x: 4.0,
            y: 4.0,
            z: 7.0,
        };
        assert_eq!(first.magnitude(), 9.0);
    }

    #[test]
    fn subtracting_two_vector3_instances_works() {
        let first = Vector3 {
            x: 5.0,
            y: 3.0,
            z: -2.0,
        };
        let second = Vector3 {
            x: 1.0,
            y: -2.0,
            z: 4.0,
        };
        assert_eq!(
            &first - &second,
            Vector3 {
                x: 4.0,
                y: 5.0,
                z: -6.0,
            }
        );
        assert_eq!(
            &second - &first,
            Vector3 {
                x: -4.0,
                y: -5.0,
                z: 6.0,
            }
        );
    }
}
