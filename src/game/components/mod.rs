use bevy::prelude::Component;

pub mod organisms;
pub mod gene;
pub mod tags;

#[derive(Component)]
pub struct EnergyLevel {
    energy: f32,
}

impl EnergyLevel {
    pub fn new(energy: f32) -> EnergyLevel {
        Self {
            energy
        }
    }
}
