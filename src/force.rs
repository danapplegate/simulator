use crate::math::Distance;
use crate::{Body, PositionVector};
use itertools::Itertools;
use std::collections::HashMap;

const G: f64 = 6.67430e-11;

#[derive(Debug)]
pub struct ForceVector<const N: usize> {
    pub label: String,
    pub v: PositionVector<N>,
}

impl<const N: usize> ForceVector<N> {
    pub fn magnitude(&self) -> f64 {
        self.v.magnitude()
    }
}

pub trait Force {
    // Returns the magnitude of the force calculated between
    // two objects of type T.
    fn calculate<'a, const N: usize>(&self, on: &'a Body<N>, from: &'a Body<N>) -> ForceVector<N>;
}

#[derive(Debug)]
pub struct Gravity {
    g: f64,
}

impl Gravity {
    pub fn new(g: Option<f64>) -> Self {
        Gravity { g: g.unwrap_or(G) }
    }

    fn bodies_key<'a, const N: usize>(b1: &Body<N>, b2: &Body<N>) -> String {
        if b1.label < b2.label {
            format!("{}_{}", b1.label, b2.label)
        } else {
            format!("{}_{}", b2.label, b1.label)
        }
    }

    pub fn forces_from_bodies<const N: usize>(&self, bodies: &Vec<&Body<N>>) -> ForceMap<N> {
        let mut force_map = ForceMap::new();
        for body_pair in bodies.iter().combinations(2) {
            let (b1, b2) = (body_pair[0], body_pair[1]);

            force_map
                .entry(b1.label.clone())
                .or_insert(vec![])
                .push(self.calculate(b1, b2));

            force_map
                .entry(b2.label.clone())
                .or_insert(vec![])
                .push(self.calculate(b2, b1));
        }
        force_map
    }
}

impl Force for Gravity {
    fn calculate<'a, const N: usize>(&self, on: &'a Body<N>, from: &'a Body<N>) -> ForceVector<N> {
        let distance = on.position.distance(&from.position);
        let magnitude = self.g * on.mass * from.mass / distance.powi(2);

        let on_force_name = format!("gravity_{}", from.label);
        let from_force_name = format!("gravity_{}", on.label);
        ForceVector {
            label: on_force_name,
            v: magnitude * &on.position.direction(&from.position),
        }
    }
}

pub type ForceMap<const N: usize> = HashMap<String, Vec<ForceVector<N>>>;
