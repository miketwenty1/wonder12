use bevy::prelude::*;

#[derive(Resource)]
pub struct CapitalizeToggle(pub bool);

#[derive(Resource, Clone)]
pub struct KeyboardData(pub String);
