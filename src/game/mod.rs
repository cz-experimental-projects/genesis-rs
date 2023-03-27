pub(crate) mod ui;
pub(crate) mod components;
pub(crate) mod plugins;
pub(crate) mod events;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::AppState;

use self::{plugins::{cam::GenesisCameraControlPlugin, evt::GenesisEventHandlerPlugin}, events::new_organism::NewOrganismEvent, components::{organism::{Organ, OrganType, DNA}, gene::{Gene, Shape}}};

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Macroworld,
    Microworld,
    Organism,
}

pub struct GenesisGamePlugin;
impl Plugin for GenesisGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4);
        app.add_state::<GameState>();
        app.add_plugin(ShapePlugin);
        app.add_plugin(GenesisCameraControlPlugin);
        app.add_plugin(GenesisEventHandlerPlugin);
        app.add_system(test.run_if(in_state(AppState::Game)));
    }
}

fn test(mut evt_writer: EventWriter<NewOrganismEvent>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::G) {
        evt_writer.send(NewOrganismEvent::new(vec![
            Organ { 
                dna: DNA::new(vec![Gene::Shape(Shape::Circle(20.0)), Gene::Color(Color::RED)]), 
                organ_type: OrganType::InformationProcessing 
            }
        ]));
    }
}