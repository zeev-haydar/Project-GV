use bevy::color::palettes::css::MAROON;
use bevy::color::palettes::tailwind::LIME_200;
use bevy::prelude::*;

/**
GameState store information about game state.
 */
#[derive(Debug, Resource)]
pub struct GameState {
    pub position: Vec3,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            position: Vec3::ZERO,
        }
    }
}

// WorldAttribute for world environment parameter

/**
This struct store attribute for the world
 */
#[derive(Debug, Resource)]
pub struct WorldAttribute {
    pub width: f32,
    pub height: f32,
    pub wall_height: f32,
    pub wall_thickness: f32,
    pub ground_color: Color,
    pub wall_color: Color,
}

impl Default for WorldAttribute {
    fn default() -> Self {
        WorldAttribute {
            width: 100.,
            height: 100.,
            wall_height: 50.,
            wall_thickness: 5.,
            wall_color: Color::from(MAROON),
            ground_color: Color::from(LIME_200)
        }
    }
}