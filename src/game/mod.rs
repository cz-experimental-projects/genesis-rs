pub(crate) mod ui;
pub(crate) mod components;
pub(crate) mod plugins;
pub(crate) mod events;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::AppState;

use self::{plugins::{camera_control::GenesisCameraControlPlugin, event_handler::GenesisEventHandlerPlugin, state_transition::GenesisGameStateTransitionPlugin}, events::new_organism::NewMacroOrganismEvent, components::{organisms::macro_organism::{Organ, OrganType}, gene::{Gene, Shape, DNA}}};

#[derive(States, Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GameState {
    #[default]
    Macroworld,
    Microworld,
}

#[derive(Resource)]
pub struct IsMouseOverUI(bool);

pub struct GenesisGamePlugin;
impl Plugin for GenesisGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Sample4);
        app.insert_resource(IsMouseOverUI(false));
        app.add_state::<GameState>();
        app.add_plugin(ShapePlugin);
        app.add_plugin(GenesisCameraControlPlugin);
        app.add_plugin(GenesisEventHandlerPlugin);
        app.add_plugin(GenesisGameStateTransitionPlugin);
        app.add_system(test.run_if(in_state(AppState::Game)));
    }
}

fn test(mut evt_writer: EventWriter<NewMacroOrganismEvent>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::G) {
        evt_writer.send(NewMacroOrganismEvent::new(vec![
            Organ { 
                dna: DNA::new(vec![Gene::Shape(Shape::Circle(20.0)), Gene::Color(Color::RED)]), 
                organ_type: OrganType::default(),
            }
        ]));
    }
}
