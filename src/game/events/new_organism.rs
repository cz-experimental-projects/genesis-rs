use bevy::prelude::Transform;

use crate::game::components::organisms::{macro_organism::{Organs, Organ}, micro_organaism::{Cell, Cells}};

#[derive(Default, Debug)]
pub struct NewMacroOrganismEvent {
    pub organism: Organs,
    pub transform: Transform,
}

impl NewMacroOrganismEvent {
    pub fn new(organs: Vec<Organ>) -> NewMacroOrganismEvent {
        Self {
            organism: Organs::new(organs),
            ..Default::default()
        }
    }
}

#[derive(Default, Debug)]
pub struct NewMicroOrganismEvent {
    pub cells: Cells,
    pub transform: Transform,
}

impl NewMicroOrganismEvent {
    pub fn new(cells: Vec<Cell>) -> NewMicroOrganismEvent {
        Self {
            cells: Cells::new(cells),
            ..Default::default()
        }
    }
}
