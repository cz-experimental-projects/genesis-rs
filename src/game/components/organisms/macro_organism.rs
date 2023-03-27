use bevy::prelude::Component;

use crate::game::components::gene::DNA;

#[derive(Clone, Debug, Default)]
pub enum OrganType {
    #[default]
    InformationProcessing,
    Locomotion,
    Prehensing,
    Sensory,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Organ {
    pub dna: DNA,
    pub organ_type: OrganType,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Organs { 
    pub organs: Vec<Organ> 
}

impl Organs {
    pub fn new(organs: Vec<Organ>) -> Organs {
        Self {
            organs
        }
    }
}
