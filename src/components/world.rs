use std::ops::Mul;
use bevy::prelude::*;

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

use bevy::prelude::*;

#[derive(Component)]
pub struct AabbCollider {
    /// Half the dimensions of the box along each axis.
    pub half_extents: Vec3,
}

pub fn ray_intersect_aabb(
    ray_origin: Vec3,
    ray_direction: Vec3,
    box_center: Vec3,
    half_extents: Vec3,
) -> Option<f32> {
    // Compute the slab intersections for each axis.
    let inv_dir = Vec3::new(
        1.0 / ray_direction.x,
        1.0 / ray_direction.y,
        1.0 / ray_direction.z,
    );
    let t1 = ((box_center - half_extents) - ray_origin).mul(inv_dir);
    let t2 = ((box_center + half_extents) - ray_origin).mul(inv_dir);

    // For each axis, t_min is the entry and t_max is the exit distance.
    let t_min = Vec3::new(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
    let t_max = Vec3::new(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

    // The overall entry point is the maximum of the three t_mins.
    let t_enter = t_min.x.max(t_min.y).max(t_min.z);
    // The overall exit point is the minimum of the three t_maxes.
    let t_exit = t_max.x.min(t_max.y).min(t_max.z);

    // If there is no valid intersection, return None.
    if t_enter > t_exit || t_exit < 0.0 {
        None
    } else {
        Some(t_enter)
    }
}