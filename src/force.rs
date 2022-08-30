use crate::math::Distance;
use crate::{Body, Massive, Positioned, Vector2};
use std::collections::HashMap;

const G: f64 = 6.67430e-11;

#[derive(Debug)]
pub struct ForceVector<T: Distance> {
    label: String,
    v: T,
}

impl<T: Distance> ForceVector<T> {
    pub fn magnitude(&self) -> f64 {
        self.v.magnitude()
    }
}

pub trait Force<'a> {
    // Returns the magnitude of the force calculated between
    // two objects of type T.
    fn calculate(&mut self, on: &'a Body<'a>, from: &'a Body<'a>) -> ForceVector<Vector2>;
}

pub struct Gravity {
    magnitudes: HashMap<String, f64>,
}

impl Gravity {
    pub fn new() -> Self {
        Gravity {
            magnitudes: HashMap::new(),
        }
    }

    fn bodies_key<'a>(b1: &Body<'a>, b2: &Body<'a>) -> String {
        if b1.label < b2.label {
            format!("{}_{}", b1.label, b2.label)
        } else {
            format!("{}_{}", b2.label, b1.label)
        }
    }
}

impl<'a> Force<'a> for Gravity {
    fn calculate(&mut self, on: &'a Body<'a>, from: &'a Body<'a>) -> ForceVector<Vector2> {
        let body_key = Self::bodies_key(on, from);
        let mut magnitude: f64 = 0.0;
        let l_position = on.position();
        let r_position = from.position();
        if !self.magnitudes.contains_key(&body_key) {
            let distance = l_position.distance(&r_position);
            magnitude = G * on.mass() * from.mass() / distance.powi(2);
            self.magnitudes.insert(body_key, magnitude);
        } else {
            magnitude = match self.magnitudes.get(&body_key) {
                Some(m) => *m,
                None => 0.0,
            };
        }
        let force_name = format!("gravity_{}", from.label);
        let direction = &l_position.direction(&r_position);
        ForceVector {
            label: force_name,
            v: magnitude * direction,
        }
    }
}
