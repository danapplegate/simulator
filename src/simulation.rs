use crate::force::{ForceVector, Gravity};
use crate::math::vector::{Distance, Vector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::mem;

pub type PositionVector<const N: usize> = Vector<N>;
pub type VelocityVector<const N: usize> = Vector<N>;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct SpinCharacteristics<const N: usize> {
    pub tilt: f32,
    pub velocity: f32,
    pub angle: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<const N: usize> {
    pub label: String,
    pub mass: f32,
    pub diameter: f32,
    #[serde(default)]
    pub position: PositionVector<N>,
    #[serde(default)]
    pub velocity: VelocityVector<N>,
    #[serde(default)]
    pub spin: SpinCharacteristics<N>,

    #[serde(skip)]
    pub forces: Vec<ForceVector<N>>,
}

impl<const N: usize> Body<N> {
    pub fn new(
        label: String,
        mass: f32,
        diameter: f32,
        position: PositionVector<N>,
        velocity: VelocityVector<N>,
        spin: SpinCharacteristics<N>,
    ) -> Self {
        Self {
            label,
            mass,
            diameter,
            position,
            velocity,
            spin,
            forces: Vec::new(),
        }
    }

    fn apply_forces(&mut self, t_step: f32) {
        let net_force: Vector<N> = self.forces.iter().map(|f| f.v).sum();
        let acceleration = net_force.magnitude() / self.mass;
        let acceleration_vector = acceleration * &net_force.normalize();
        let displacement =
            &(t_step * &self.velocity) + &(0.5 * t_step.powi(2) * &acceleration_vector);
        self.position = &self.position + &displacement;
        self.velocity = &self.velocity + &(t_step * &acceleration_vector);
    }

    fn apply_spin(&mut self, t_step: f32) {
        self.spin.angle = self.spin.angle + (t_step * self.spin.velocity);
    }
}

pub type BodyMap<const N: usize> = BTreeMap<String, Body<N>>;

fn body_map_from_bodies<'a, const N: usize>(bodies: &'a Vec<Body<N>>) -> BodyMap<N> {
    let mut body_map = BodyMap::new();
    for body in bodies {
        body_map.insert(
            String::from(&body.label),
            Body::new(
                body.label.clone(),
                body.mass,
                body.diameter,
                body.position,
                body.velocity,
                body.spin,
            ),
        );
    }
    body_map
}

fn compute_next_step<const N: usize>(body_map: &BodyMap<N>, t_step: f32) -> BodyMap<N> {
    let g = Gravity::new(None);
    let mut new_body_map = BodyMap::new();
    let mut bodies = Vec::new();
    for body in body_map.values() {
        bodies.push(body);
    }
    let mut force_map = g.forces_from_bodies(&bodies);
    for body in bodies {
        let mut new_body = Body {
            label: body.label.clone(),
            forces: force_map.remove(&body.label).unwrap_or_default(),
            ..*body
        };

        new_body.apply_forces(t_step);
        new_body.apply_spin(t_step);
        new_body_map.insert(body.label.clone(), new_body);
    }

    new_body_map
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simulation<const N: usize> {
    bodies: Vec<Body<N>>,
    t_start: f32,
    t_end: Option<f32>,
    t_step: f32,
}

impl<const N: usize> Simulation<N> {
    pub fn new(t_start: Option<f32>, t_end: Option<f32>, t_step: Option<f32>) -> Self {
        Self {
            bodies: Vec::new(),
            t_start: t_start.unwrap_or(0.0),
            t_end: t_end,
            t_step: t_step.unwrap_or(0.1),
        }
    }

    pub fn add_body(&mut self, body: Body<N>) {
        self.bodies.push(body)
    }

    pub fn create_body_map<'a>(&'a self) -> BodyMap<N> {
        body_map_from_bodies(&self.bodies)
    }

    pub fn bodies(&self) -> &Vec<Body<N>> {
        &self.bodies
    }
}

pub struct RunStep<const N: usize> {
    pub t: f32,
    pub body_map: BodyMap<N>,
}

pub struct Run<'a, const N: usize> {
    simulation: &'a Simulation<N>,
    t_current: f32,
    body_map: BodyMap<N>,
}

impl<'a, const N: usize> From<&'a Simulation<N>> for Run<'a, N> {
    fn from(simulation: &'a Simulation<N>) -> Self {
        Self {
            simulation,
            t_current: simulation.t_start,
            body_map: simulation.create_body_map(),
        }
    }
}

impl<'a, const N: usize> Iterator for Run<'a, N> {
    type Item = RunStep<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t_end) = self.simulation.t_end {
            if self.t_current > t_end {
                return None;
            }
        }
        let next_body_map = compute_next_step(&self.body_map, self.simulation.t_step);
        let t = self.t_current;
        self.t_current += self.simulation.t_step;
        Some(Self::Item {
            t,
            body_map: mem::replace(&mut self.body_map, next_body_map),
        })
    }
}

/// Version of a simulation run that takes ownership of the simulation
pub struct OwningRun<const N: usize> {
    simulation: Simulation<N>,
    t_current: f32,
    body_map: BodyMap<N>,
}

impl<const N: usize> From<Simulation<N>> for OwningRun<N> {
    fn from(simulation: Simulation<N>) -> Self {
        let t_current = simulation.t_start;
        let body_map = simulation.create_body_map();
        Self {
            simulation,
            t_current,
            body_map,
        }
    }
}

impl<const N: usize> Iterator for OwningRun<N> {
    type Item = RunStep<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t_end) = self.simulation.t_end {
            if self.t_current > t_end {
                return None;
            }
        }
        let next_body_map = compute_next_step(&self.body_map, self.simulation.t_step);
        let t = self.t_current;
        self.t_current += self.simulation.t_step;
        Some(Self::Item {
            t,
            body_map: mem::replace(&mut self.body_map, next_body_map),
        })
    }
}
