use crate::force::{ForceVector, Gravity};
use crate::math::vector::{Distance, Vector};
use std::collections::HashMap;

pub type PositionVector<const N: usize> = Vector<N>;
pub type VelocityVector<const N: usize> = Vector<N>;

#[derive(Debug)]
pub struct Body<const N: usize> {
    pub label: String,
    pub mass: f64,
    pub position: PositionVector<N>,
    pub velocity: VelocityVector<N>,
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

pub type BodyMap<'a, const N: usize> = HashMap<String, Body<N>>;

fn body_map_from_bodies<'a, const N: usize>(bodies: &'a Vec<Body<N>>) -> BodyMap<'a, N> {
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

pub fn compute_next_step<const N: usize>(body_map: BodyMap<'_, N>, t_step: f64) -> BodyMap<'_, N> {
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

    pub fn t_start(&self) -> f64 {
        self.t_start
    }

    pub fn t_end(&self) -> f64 {
        self.t_end
    }

    pub fn t_step(&self) -> f64 {
        self.t_step
    }

    pub fn add_body(&mut self, body: Body<N>) {
        self.bodies.push(body)
    }

    pub fn create_body_map<'a>(&'a self) -> BodyMap<'a, N> {
        body_map_from_bodies(&self.bodies)
    }

    pub fn bodies(&self) -> &Vec<Body<N>> {
        &self.bodies
    }
}
