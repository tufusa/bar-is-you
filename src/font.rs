use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Title(pub Handle<Font>);

#[derive(Resource, Clone)]
pub struct UI(pub Handle<Font>);

#[derive(Resource, Clone)]
pub struct Rule(pub Handle<Font>);
