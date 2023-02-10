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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_from_an_array_works() {
        let _ = Matrix4::from([0.0; 16]);
    }
}
