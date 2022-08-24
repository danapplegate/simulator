pub mod force;
pub mod math;
pub mod output_adapter;

use crate::math::vector::{Vector1, Vector2};
use std::collections::HashMap;

pub trait Massive {
    fn mass(&self) -> f64;
}

pub trait Positioned {
    type Output;
    fn position(self) -> Self::Output;
}

pub type PositionVector1 = Vector1;
pub type PositionVector2 = Vector2;

#[derive(Debug)]
pub struct Body<'a> {
    pub label: &'a str,
    mass: f64,
    position: PositionVector2,
}

impl<'a> Massive for Body<'a> {
    fn mass(&self) -> f64 {
        self.mass
    }
}

impl<'a> Positioned for &'a Body<'a> {
    type Output = &'a PositionVector2;
    fn position(self) -> Self::Output {
        &self.position
    }
}

impl<'a> Body<'a> {
    pub fn new(label: &'a str, mass: f64, position: Option<PositionVector2>) -> Self {
        let p = position.unwrap_or_default();
        Self {
            label,
            mass,
            position: p,
        }
    }
}

type BodyStates<'a> = HashMap<&'a str, Body<'a>>;

#[derive(Debug)]
pub struct SimulationStep<'a> {
    t: f64,
    body_states: BodyStates<'a>,
}

impl<'a> SimulationStep<'a> {
    fn body_states(&'a self) -> &'a BodyStates<'a> {
        &self.body_states
    }
}

pub struct SimulationRun<'a> {
    t_step: f64,
    t_end: f64,
    t_current: f64,
    simulation: &'a Simulation<'a>,
}

impl<'a> SimulationRun<'a> {
    fn compute_body_states(&self) -> BodyStates<'a> {
        let mut body_states = BodyStates::new();
        for body in self.simulation.bodies.iter() {
            let body_state = Body {
                position: &body.position
                    + &PositionVector2 {
                        x: 0.0,
                        y: -0.5_f64 * self.t_current.powi(2) * 9.81_f64,
                    },
                mass: body.mass,
                label: body.label,
            };
            body_states.insert(body.label, body_state);
        }
        body_states
    }
}

impl<'a> Iterator for SimulationRun<'a> {
    type Item = SimulationStep<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.t_current += self.t_step;
        if self.t_current > self.t_end {
            None
        } else {
            Some(SimulationStep {
                t: self.t_current,
                body_states: self.compute_body_states(),
            })
        }
    }
}

#[derive(Debug)]
pub struct Simulation<'a> {
    bodies: Vec<Body<'a>>,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body<'a>) {
        self.bodies.push(body);
    }

    pub fn bodies(&self) -> &Vec<Body<'a>> {
        &self.bodies
    }

    pub fn new_run(&self) -> SimulationRun {
        SimulationRun {
            t_step: 0.1,
            t_end: 10.0,
            t_current: 0.0,
            simulation: &self,
        }
    }
}
