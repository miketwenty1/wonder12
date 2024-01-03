use bevy::prelude::*;

use crate::resourcey::TargetType;

#[derive(Resource)]
pub struct CapitalizeToggle(pub bool);

#[derive(Resource, Clone)]
pub struct KeyboardData {
    pub value: String,
    pub target: TargetType,
}
