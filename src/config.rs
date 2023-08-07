use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{graphics::model::Model, simulation::Simulation};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config<const N: usize> {
    pub simulation: Simulation<N>,
    pub models: HashMap<String, Model>,
}
