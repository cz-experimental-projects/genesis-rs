use bevy::prelude::{Plugin, Query, OrthographicProjection, With, IntoSystemConfig, in_state, Res, State, Commands, NextState, Entity, IntoSystemAppConfig, OnExit, Visibility, OnEnter, DespawnRecursiveExt};

use crate::{game::{components::{tags::{MainCameraTag, MacroOrganismTag, MicroOrganismTag}, organisms::macro_organism::Organs, gene::{Gene, Shape}}, GameState}, AppState};

pub struct GenesisGameStateTransitionPlugin;
impl Plugin for GenesisGameStateTransitionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(check_transition.run_if(in_state(AppState::Game)));
        app.add_system(despawn_macro_organisms.in_schedule(OnExit(GameState::Macroworld)));
        app.add_system(spawn_macro_organisms.in_schedule(OnEnter(GameState::Macroworld)));
        app.add_system(despawn_micro_organisms.in_schedule(OnExit(GameState::Microworld)));
        app.add_system(spawn_micro_organisms.in_schedule(OnEnter(GameState::Microworld)));
    }
}

fn check_transition(mut commands: Commands, mut camera: Query<&mut OrthographicProjection, With<MainCameraTag>>, state: Res<State<GameState>>) {
    let mut camera = camera.single_mut();

    if camera.scale <= 0.05 {
        if state.0 == GameState::Macroworld {
            commands.insert_resource(NextState(Some(GameState::Microworld)));
            camera.scale = 0.75;
            println!("Entered micro world");
        }
    }

    if camera.scale >= 8.0 {
        if state.0 == GameState::Microworld {
            commands.insert_resource(NextState(Some(GameState::Macroworld))); 
            camera.scale = 0.1;
            println!("Entered macro world");
        }
    }
}

fn despawn_macro_organisms(mut commands: Commands, organisms: Query<Entity, With<MacroOrganismTag>>) {
    for entity in organisms.iter() {
        commands.entity(entity)
            .insert(Visibility::Hidden);
    }
}

fn spawn_macro_organisms(mut commands: Commands, organisms: Query<Entity, With<MacroOrganismTag>>) {
    for entity in organisms.iter() {
        commands.entity(entity)
            .insert(Visibility::default());
    }
}

fn despawn_micro_organisms(mut commands: Commands, organisms: Query<Entity, With<MicroOrganismTag>>) {
    for entity in organisms.iter() {
        commands.entity(entity)
            .despawn_recursive();
    }
}

fn spawn_micro_organisms(mut commands: Commands, organisms: Query<&Organs, With<MacroOrganismTag>>) {
    for organism in organisms.iter() {
        for organ in &organism.organs {
            for gene in &organ.dna.dna {
                if let Gene::Shape(shape) = gene {
                    match shape {
                        Shape::Circle(radius) => todo!(),
                        Shape::Polygon(sides, radius) => todo!(),
                    }
                }
            }
        }
    }
}
