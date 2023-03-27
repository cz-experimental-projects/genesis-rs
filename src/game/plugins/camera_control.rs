use bevy::{prelude::{Plugin, Query, Res, Input, KeyCode, OnUpdate, IntoSystemConfigs, EventReader, OrthographicProjection, With, Transform, Vec3}, input::mouse::MouseWheel};

use crate::{AppState, game::{components::tags::MainCameraTag, IsMouseOverUI}, utils::math};

pub struct GenesisCameraControlPlugin;
impl Plugin for GenesisCameraControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems((
            camera_zoom, 
            camera_movement
        ).in_set(OnUpdate(AppState::Game)));
    }
}

const ZOOM_CONSTANT_SMALL: f32 = 0.025;
const ZOOM_CONSTANT_LARGE: f32 = 0.4;
const MOVE_CONSANT: f32 = 12.0;

fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection, With<MainCameraTag>>, 
    mut input: EventReader<MouseWheel>,
    mouse_over_ui: Res<IsMouseOverUI>
) {
    if mouse_over_ui.0 { return; }

    let event = input.iter().last();
    if let Some(event) = event { 
        if event.y == 0.0 { return; }

        let mut camera = camera.single_mut();
        let constant = if camera.scale < 1.0 { ZOOM_CONSTANT_SMALL } else { ZOOM_CONSTANT_LARGE }; 
        
        camera.scale = math::clamp(camera.scale - event.y * constant, 0.05, 10.0);
    }
}

fn camera_movement(
    mut camera: Query<(&OrthographicProjection, &mut Transform), With<MainCameraTag>>, 
    input: Res<Input<KeyCode>>,
    mouse_over_ui: Res<IsMouseOverUI>
) {
    if mouse_over_ui.0 { return; }

    let mut dir = Vec3::new(0.0, 0.0, 0.0);

    if input.pressed(KeyCode::W) {
        dir.y += 1.0; 
    }

    if input.pressed(KeyCode::S) {
        dir.y -= 1.0;
    }

    if input.pressed(KeyCode::A) {
        dir.x -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        dir.x += 1.0;
    }

    if dir == Vec3::ZERO {
        return;
    }

    let camera = camera.single_mut();

    let orthographic_projection = camera.0;
    let mut camera = camera.1;
    let dir = dir.normalize();
    
    camera.translation += MOVE_CONSANT * dir * orthographic_projection.scale;
}
