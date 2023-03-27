use bevy::prelude::{Plugin, EventReader, Commands, IntoSystemConfig, in_state, BuildChildren, GlobalTransform, Visibility, ComputedVisibility};
use bevy_prototype_lyon::{prelude::*, shapes::Circle};

use crate::{game::{events::new_organism::NewOrganismEvent, components::{organism::OrganismTag, gene::{Gene, Shape}}}, AppState};

pub struct GenesisEventHandlerPlugin;
impl Plugin for GenesisEventHandlerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<NewOrganismEvent>();
        app.add_system(add_organism.run_if(in_state(AppState::Game)));
    }
}

fn add_organism(mut commands: Commands, mut event: EventReader<NewOrganismEvent>) {
    for ev in event.iter() {
        commands.spawn((
            OrganismTag,
            GlobalTransform::default(),
            ev.transform,
            Visibility::default(),
            ComputedVisibility::default(),
        )).with_children(|parent| {
            for organ in &ev.organism.organs {
                let mut organ_entity = parent.spawn(organ.dna.clone());
                
                for gene in &organ.dna.dna {
                    match gene {
                        Gene::Shape(shape) => {
                            match shape {
                                Shape::Circle(radius) => {
                                    organ_entity.insert(ShapeBundle{
                                        path: GeometryBuilder::build_as(&Circle { radius: radius.clone(), ..Circle::default() }),
                                        ..ShapeBundle::default()
                                    });
                                },
                                Shape::Polygon(sides, radius) => {
                                    organ_entity.insert(ShapeBundle{
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
                        Gene::Color(color) => { organ_entity.insert(Fill::color(color.clone())); },
                        Gene::Stroke(color, width) => { organ_entity.insert(Stroke::new(color.clone(), width.clone())); },
                    }
                }
            }
        });
    }
}
