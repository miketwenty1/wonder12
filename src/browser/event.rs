use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ReadLocalBrowserStorage;

#[derive(Event, Debug)]
pub struct ReadIndexedDBStorage;

#[derive(Event, Debug)]
pub struct WriteLocalBrowserStorage;
