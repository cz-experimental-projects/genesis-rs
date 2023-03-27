use bevy::{prelude::{Commands, EventWriter, NextState}, app::AppExit};
use bevy_egui::EguiContexts;
use bevy_inspector_egui::egui::{self, RichText};

use crate::AppState;

pub fn main_menu(mut contexts: EguiContexts, mut commands: Commands, mut quit: EventWriter<AppExit>) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| ui.vertical_centered(|ui| {
        ui.label(RichText::new("Genesis").size(100.0));
        
        if ui.button(RichText::new("Start").size(40.0)).clicked() {
            commands.insert_resource(NextState(Some(AppState::Game)));
        }

        ui.add_space(20.0);

        if ui.button(RichText::new("Quit").size(40.0)).clicked() {
            quit.send(AppExit);
        }
    }));
}
