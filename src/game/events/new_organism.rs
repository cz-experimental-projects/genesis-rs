use bevy::prelude::Transform;

use crate::game::components::organism::{Organs, Organ};

#[derive(Default, Debug)]
pub struct NewOrganismEvent {
    pub organism: Organs,
    pub transform: Transform,
}

impl NewOrganismEvent {
    pub fn new(organs: Vec<Organ>) -> NewOrganismEvent {
        Self {
            organism: Organs::new(organs),
            ..Default::default()
        }
    }
}
