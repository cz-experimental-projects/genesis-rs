use bevy::prelude::{Plugin, IntoSystemConfig, in_state};

use crate::AppState;

mod main_menu;

pub struct GenesisUIPlugin;
impl Plugin for GenesisUIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(main_menu::main_menu.run_if(in_state(AppState::MainMenu)));
    }
}
