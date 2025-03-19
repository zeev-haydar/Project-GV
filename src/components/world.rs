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

#[derive(Component)]
pub struct AabbCollider {
    /// Half the dimensions of the box along each axis.
    pub half_extents: Vec3,
}

// pub fn ray_intersect_aabb(
//     ray_origin: Vec3,
//     ray_direction: Vec3,
//     box_center: Vec3,
//     half_extents: Vec3,
// ) -> Option<f32> {
//     // Compute the slab intersections for each axis.
//     let inv_dir = Vec3::new(
//         1.0 / ray_direction.x,
//         1.0 / ray_direction.y,
//         1.0 / ray_direction.z,
//     );
//     let t1 = ((box_center - half_extents) - ray_origin).mul(inv_dir);
//     let t2 = ((box_center + half_extents) - ray_origin).mul(inv_dir);

//     // For each axis, t_min is the entry and t_max is the exit distance.
//     let t_min = Vec3::new(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
//     let t_max = Vec3::new(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

//     // The overall entry point is the maximum of the three t_mins.
//     let t_enter = t_min.x.max(t_min.y).max(t_min.z);
//     // The overall exit point is the minimum of the three t_maxes.
//     let t_exit = t_max.x.min(t_max.y).min(t_max.z);

//     // If there is no valid intersection, return None.
//     if t_enter > t_exit || t_exit < 0.0 {
//         None
//     } else {
//         Some(t_enter)
//     }
// }

fn point_inside_aabb(point: Vec3, box_center: Vec3, half_extents: Vec3, epsilon: f32) -> bool {
    let min = box_center - half_extents - Vec3::splat(epsilon);
    let max = box_center + half_extents + Vec3::splat(epsilon);
    point.cmple(max).all() && point.cmpge(min).all()
}

/// Performs a single ray–AABB intersection test.
/// Returns the distance along the ray (if hit), or None.
pub fn ray_intersect_aabb(
    ray_origin: Vec3,
    ray_direction: Vec3,
    box_center: Vec3,
    half_extents: Vec3,
) -> Option<f32> {
    // Check if the ray origin is already inside (or very near) the AABB.
    if point_inside_aabb(ray_origin, box_center, half_extents, 0.001) {
        return Some(0.0);
    }
    
    // Compute inverse direction components.
    let inv_dir = Vec3::new(
        1.0 / ray_direction.x,
        1.0 / ray_direction.y,
        1.0 / ray_direction.z,
    );
    let t1 = ((box_center - half_extents) - ray_origin).mul(inv_dir);
    let t2 = ((box_center + half_extents) - ray_origin).mul(inv_dir);

    let t_min = Vec3::new(t1.x.min(t2.x), t1.y.min(t2.y), t1.z.min(t2.z));
    let t_max = Vec3::new(t1.x.max(t2.x), t1.y.max(t2.y), t1.z.max(t2.z));

    let t_enter = t_min.x.max(t_min.y).max(t_min.z);
    let t_exit = t_max.x.min(t_max.y).min(t_max.z);

    if t_enter > t_exit || t_exit < 0.0 {
        None
    } else {
        Some(t_enter)
    }
}

/// Casts downward rays from sample points on the bottom face of the player's box (collider)
/// and checks if they hit the ground's AABB.
/// - `player_center` and `player_half_extents`: define the player's box.
/// - `ground_center` and `ground_half_extents`: define the ground's AABB.
/// - `step_size`: spacing between sample points (e.g. 0.1 units).
/// - `ray_direction`: should be downward (e.g. Vec3::NEG_Y).
///
/// Returns the smallest positive hit distance found.
pub fn multi_ray_intersect_from_box(
    player_center: Vec3,
    player_half_extents: Vec3,
    ground_center: Vec3,
    ground_half_extents: Vec3,
    step_size: f32,
    ray_direction: Vec3,
) -> Option<f32> {
    let mut closest_intersection: Option<f32> = None;

    // The bottom face of the player's box is at:
    let y_bottom = player_center.y - player_half_extents.y;
    // The bottom face spans these x and z ranges:
    let x_min = player_center.x - player_half_extents.x;
    let x_max = player_center.x + player_half_extents.x;
    let z_min = player_center.z - player_half_extents.z;
    let z_max = player_center.z + player_half_extents.z;

    let mut x = x_min;
    while x <= x_max {
        let mut z = z_min;
        while z <= z_max {
            // Sample point on player's bottom face.
            let origin = Vec3::new(x, y_bottom, z);
            if let Some(t) = ray_intersect_aabb(origin, ray_direction, ground_center, ground_half_extents) {
                // Only consider positive intersections.
                if t >= 0.0 {
                    closest_intersection = Some(match closest_intersection {
                        Some(existing) => existing.min(t),
                        None => t,
                    });
                }
            }
            z += step_size;
        }
        x += step_size;
    }

    closest_intersection
}