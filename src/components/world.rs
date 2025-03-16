use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct Ground;

/**
This component is used to name an entity
*/
#[derive(Debug, Component)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Self {
        Self(String::from(name))
    }
}

/**
This component is used to label an entity as an
Structure
*/
#[derive(Debug, Component)]
pub struct Structure;

/**
This component is used to label it as NotGround,
meaning it is the non-ground part of a structure
*/
#[derive(Debug, Component)]
pub struct NotGround;