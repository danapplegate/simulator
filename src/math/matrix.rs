use std::ops::{Index, IndexMut, Mul};

#[derive(PartialEq, Debug)]
pub struct Matrix4([f32; 16]);

impl Matrix4 {
    const ZERO: Self = Self::from_array([0.0; 16]);
    #[rustfmt::skip]
    const IDENTITY: Self = Self::from_array([
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]);

    const fn from_array(array: [f32; 16]) -> Self {
        Self(array)
    }

    fn new() -> Self {
        Self::default()
    }
}

impl From<[f32; 16]> for Matrix4 {
    fn from(components: [f32; 16]) -> Self {
        Self::from_array(components)
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f32];
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[(i * 4)..((i + 1) * 4)]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[(i * 4)..((i + 1) * 4)]
    }
}

impl Mul for Matrix4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self[0][0] * rhs[0][0]
                + self[0][1] * rhs[1][0]
                + self[0][2] * rhs[2][0]
                + self[0][3] * rhs[3][0],
            self[0][0] * rhs[0][1]
                + self[0][1] * rhs[1][1]
                + self[0][2] * rhs[2][1]
                + self[0][3] * rhs[3][1],
            self[0][0] * rhs[0][2]
                + self[0][1] * rhs[1][2]
                + self[0][2] * rhs[2][2]
                + self[0][3] * rhs[3][2],
            self[0][0] * rhs[0][3]
                + self[0][1] * rhs[1][3]
                + self[0][2] * rhs[2][3]
                + self[0][3] * rhs[3][3],
            self[1][0] * rhs[0][0]
                + self[1][1] * rhs[1][0]
                + self[1][2] * rhs[2][0]
                + self[1][3] * rhs[3][0],
            self[1][0] * rhs[0][1]
                + self[1][1] * rhs[1][1]
                + self[1][2] * rhs[2][1]
                + self[1][3] * rhs[3][1],
            self[1][0] * rhs[0][2]
                + self[1][1] * rhs[1][2]
                + self[1][2] * rhs[2][2]
                + self[1][3] * rhs[3][2],
            self[1][0] * rhs[0][3]
                + self[1][1] * rhs[1][3]
                + self[1][2] * rhs[2][3]
                + self[1][3] * rhs[3][3],
            self[2][0] * rhs[0][0]
                + self[2][1] * rhs[1][0]
                + self[2][2] * rhs[2][0]
                + self[2][3] * rhs[3][0],
            self[2][0] * rhs[0][1]
                + self[2][1] * rhs[1][1]
                + self[2][2] * rhs[2][1]
                + self[2][3] * rhs[3][1],
            self[2][0] * rhs[0][2]
                + self[2][1] * rhs[1][2]
                + self[2][2] * rhs[2][2]
                + self[2][3] * rhs[3][2],
            self[2][0] * rhs[0][3]
                + self[2][1] * rhs[1][3]
                + self[2][2] * rhs[2][3]
                + self[2][3] * rhs[3][3],
            self[3][0] * rhs[0][0]
                + self[3][1] * rhs[1][0]
                + self[3][2] * rhs[2][0]
                + self[3][3] * rhs[3][0],
            self[3][0] * rhs[0][1]
                + self[3][1] * rhs[1][1]
                + self[3][2] * rhs[2][1]
                + self[3][3] * rhs[3][1],
            self[3][0] * rhs[0][2]
                + self[3][1] * rhs[1][2]
                + self[3][2] * rhs[2][2]
                + self[3][3] * rhs[3][2],
            self[3][0] * rhs[0][3]
                + self[3][1] * rhs[1][3]
                + self[3][2] * rhs[2][3]
                + self[3][3] * rhs[3][3],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_from_an_array_works() {
        let _ = Matrix4::from([0.0; 16]);
    }

    #[test]
    fn indexing_works() {
        let mat = Matrix4::IDENTITY;
        assert_eq!(mat[1][1], 1.0);
        assert_eq!(mat[2][1], 0.0);
    }

    #[test]
    fn mutable_indexing_works() {
        let mut mat = Matrix4::ZERO;
        assert_eq!(mat[0][1], 0.0);
        mat[0][1] = 1.0;
        assert_eq!(mat[0][1], 1.0);
        assert_eq!(Matrix4::ZERO[0][1], 0.0);
    }

    #[rustfmt::skip]
    #[test]
    fn multiplication_of_two_matrix4s_works() {
        let mat1: Matrix4 = [
            7.0,  14.0, 15.0, 6.0,
            4.0,  8.0,  12.0, 3.0,
            14.0, 21.0, 6.0,  9.0,
            13.0, 7.0,  6.0,  4.0,
        ].into();
        let mat2: Matrix4 = [
            5.0,  7.0,  14.0, 2.0,
            8.0,  16.0, 4.0,  9.0,
            13.0, 6.0,  8.0,  4.0,
            6.0,  3.0,  2.0,  4.0,
        ].into();
        assert_eq!(mat1 * mat2, [
            378.0, 381.0, 286.0, 224.0,
            258.0, 237.0, 190.0, 140.0,
            370.0, 497.0, 346.0, 277.0,
            223.0, 251.0, 266.0, 129.0
        ].into());
    }
}
