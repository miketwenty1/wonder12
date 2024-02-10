use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ReadLocalBrowserStorage;

#[derive(Event, Debug)]
pub struct WriteLocalBrowserStorage;
