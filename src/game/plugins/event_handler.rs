use bevy::{prelude::{Plugin, EventReader, Commands, IntoSystemConfig, in_state, BuildChildren, GlobalTransform, Visibility, ComputedVisibility, Transform}, ecs::system::EntityCommands};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};

use crate::{game::{events::new_organism::{NewMacroOrganismEvent, NewMicroOrganismEvent}, components::{gene::{Gene, Shape}, tags::{MacroOrganismTag, MicroOrganismTag}, EnergyLevel}}, AppState};

pub struct GenesisEventHandlerPlugin;
impl Plugin for GenesisEventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<NewMacroOrganismEvent>();
        app.add_event::<NewMicroOrganismEvent>();
        app.add_system(add_macro_organisms.run_if(in_state(AppState::Game)));
    }
}

fn add_macro_organisms(mut commands: Commands, mut event: EventReader<NewMacroOrganismEvent>) {
    for ev in event.iter() {
        commands.spawn((
            MacroOrganismTag,
            GlobalTransform::default(),
            ev.transform,
            Visibility::default(),
            ComputedVisibility::default(),
            ev.organism.clone(),
        )).with_children(|parent| {
            for organ in &ev.organism.organs {
                let mut organ_entity = parent.spawn(organ.dna.clone());
                apply_genes(&mut organ_entity, &organ.dna.dna);
            }
        });
    }
}

fn add_micro_organisms(mut commands: Commands, mut event: EventReader<NewMicroOrganismEvent>) {
    for ev in event.iter() {
        commands.spawn((
            MicroOrganismTag,
            GlobalTransform::default(),
            ev.transform,
            Visibility::default(),
            ComputedVisibility::default(),
            ev.cells.clone(),
        )).with_children(|parent| {
            for cell in &ev.cells.cells {
                let mut cell_entity = parent.spawn(cell.dna.clone());
                apply_genes(&mut cell_entity, &cell.dna.dna);
            }
        });
    }
}

fn apply_genes(entity_command: &mut EntityCommands, dna: &Vec<Gene>) {
    for gene in dna {
        match gene {
            Gene::Shape(shape) => {
                match shape {
                    Shape::Circle(radius) => {
                        entity_command.insert(ShapeBundle{
                            path: GeometryBuilder::build_as(&Circle { radius: radius.clone(), ..Circle::default() }),
                            ..ShapeBundle::default()
                        });
                    },
                    Shape::Polygon(sides, radius) => {
                        entity_command.insert(ShapeBundle{
                            path: GeometryBuilder::build_as(&RegularPolygon {
                                sides: sides.clone(),
                                feature: RegularPolygonFeature::Radius(radius.clone()),
                                ..RegularPolygon::default()
                            }),
                            ..ShapeBundle::default()
                        });
                    },
                }
            },
            Gene::Color(color) => { entity_command.insert(Fill::color(color.clone())); },
            Gene::Stroke(color, width) => { entity_command.insert(Stroke::new(color.clone(), width.clone())); },
            Gene::Size(size) => { entity_command.insert(Transform::from_scale(size.clone())); },
            Gene::MaxEnergyLevel(max_energy) => { entity_command.insert(EnergyLevel::new(max_energy.clone())); },
        }
    }
}
