use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_tweening::TweeningPlugin;
use game::{GenesisGamePlugin, ui::GenesisUIPlugin, components::tags::MainCameraTag};

mod game;
mod utils;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    Game,
    PausedMenu,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GenesisGamePlugin)
        .add_plugin(GenesisUIPlugin)
        .add_startup_system(setup_cam)
        .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCameraTag,
    ));
}
