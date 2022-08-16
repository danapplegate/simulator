extern crate nalgebra as na;

pub mod math;

use na::Vector2;
use std::collections::HashMap;

const GRAVITY_MAGNITUDE: f32 = 9.81;
const GRAVITY2: Vector2<f32> = Vector2::new(0.0, -GRAVITY_MAGNITUDE);

#[derive(Debug)]
pub enum SimulationType {
    Newtonian,
}

pub trait Massive {
    fn mass(&self) -> f32;
}

pub type PositionVector2 = Vector2<f32>;

#[derive(Debug)]
pub struct Body<'a> {
    pub label: &'a str,
    mass: f32,
    position: PositionVector2,
}

impl<'a> Massive for Body<'a> {
    fn mass(&self) -> f32 {
        self.mass
    }
}

impl<'a> Body<'a> {
    pub fn new(label: &'a str, mass: f32, position: Option<PositionVector2>) -> Self {
        let p = position.unwrap_or(PositionVector2::new(0.0, 0.0));
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
    t: f32,
    body_states: BodyStates<'a>,
}

pub struct SimulationRun<'a> {
    t_start: f32,
    t_step: f32,
    t_end: f32,
    t_current: f32,
    simulation: &'a Simulation<'a>,
}

impl<'a> SimulationRun<'a> {
    fn compute_body_states(&self) -> BodyStates<'a> {
        let mut body_states = BodyStates::new();
        for body in self.simulation.bodies.iter() {
            let body_state = Body {
                position: body.position
                    + PositionVector2::new(0.0, -0.5 * self.t_current * 9.81 * 9.81),
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
    simulation_type: SimulationType,
    dimensions: u32,
    bodies: Vec<Body<'a>>,
}

impl<'a> Simulation<'a> {
    pub fn new(simulation_type: SimulationType, dimensions: u32) -> Self {
        Self {
            simulation_type,
            dimensions,
            bodies: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body<'a>) {
        self.bodies.push(body);
    }

    pub fn new_run(&self) -> SimulationRun {
        SimulationRun {
            t_start: 0.0,
            t_step: 0.1,
            t_end: 10.0,
            t_current: 0.0,
            simulation: &self,
        }
    }
}
