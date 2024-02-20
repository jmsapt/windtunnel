use bevy::{
    app::Plugin,
    log::{debug, info},
};

pub struct Solver;

impl Plugin for Solver {
    fn build(&self, app: &mut bevy::prelude::App) {
        dbg!("Building solver...");
    }
}
