use crate::swarm_element::SwarmElement;

pub trait Measurements {
    fn ranging(&self, swarm_element: &SwarmElement, std_raning: f64) -> f64;
}
