use bevy::prelude::Component;

#[derive(Component)]
pub struct EnergyLevel {
    // should be between 0 and 1
    energy: f32,
}


