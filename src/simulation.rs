use crate::force::{ForceVector, Gravity};
use crate::math::vector::{Distance, Vector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::mem;

pub type PositionVector<const N: usize> = Vector<N>;
pub type VelocityVector<const N: usize> = Vector<N>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<const N: usize> {
    pub label: String,
    pub mass: f64,
    pub position: PositionVector<N>,
    pub velocity: VelocityVector<N>,

    #[serde(skip)]
    pub forces: Vec<ForceVector<N>>,
}

impl<const N: usize> Body<N> {
    pub fn new(
        label: String,
        mass: f64,
        position: Option<PositionVector<N>>,
        velocity: Option<VelocityVector<N>>,
    ) -> Self {
        let p = position.unwrap_or_default();
        let v = velocity.unwrap_or_default();
        Self {
            label,
            mass,
            position: p,
            velocity: v,
            forces: Vec::new(),
        }
    }

    fn apply_forces(&mut self, t_step: f64) {
        let net_force: Vector<N> = self.forces.iter().map(|f| f.v).sum();
        let acceleration = net_force.magnitude() / self.mass;
        let acceleration_vector = acceleration * &net_force.unit();
        let displacement =
            &(t_step * &self.velocity) + &(0.5 * t_step.powi(2) * &acceleration_vector);
        self.position = &self.position + &displacement;
        self.velocity = &self.velocity + &(t_step * &acceleration_vector);
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
                Some(body.position),
                Some(body.velocity),
            ),
        );
    }
    body_map
}

fn compute_next_step<const N: usize>(body_map: &BodyMap<N>, t_step: f64) -> BodyMap<N> {
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
        new_body_map.insert(body.label.clone(), new_body);
    }

    new_body_map
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Simulation<const N: usize> {
    bodies: Vec<Body<N>>,
    t_start: f64,
    t_end: f64,
    t_step: f64,
}

impl<const N: usize> Simulation<N> {
    pub fn new(t_start: Option<f64>, t_end: Option<f64>, t_step: Option<f64>) -> Self {
        Self {
            bodies: Vec::new(),
            t_start: t_start.unwrap_or(0.0),
            t_end: t_end.unwrap_or(10.0),
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
    pub t: f64,
    pub body_map: BodyMap<N>,
}

pub struct Run<'a, const N: usize> {
    simulation: &'a Simulation<N>,
    t_current: f64,
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
        if self.t_current > self.simulation.t_end {
            None
        } else {
            let next_body_map = compute_next_step(&self.body_map, self.simulation.t_step);
            let t = self.t_current;
            self.t_current += self.simulation.t_step;
            Some(Self::Item {
                t,
                body_map: mem::replace(&mut self.body_map, next_body_map),
            })
        }
    }
}

/// Version of a simulation run that takes ownership of the simulation
pub struct OwningRun<const N: usize> {
    simulation: Simulation<N>,
    t_current: f64,
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
        if self.t_current > self.simulation.t_end {
            None
        } else {
            let next_body_map = compute_next_step(&self.body_map, self.simulation.t_step);
            let t = self.t_current;
            self.t_current += self.simulation.t_step;
            Some(Self::Item {
                t,
                body_map: mem::replace(&mut self.body_map, next_body_map),
            })
        }
    }
}
