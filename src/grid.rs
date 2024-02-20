use std::fmt::Debug;

use bevy::{
    asset::Asset,
    ecs::{storage, system::Resource},
    math::{Vec2, Vec3},
    pbr::Material,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    ui::debug,
};

/// Provides extensible grid functionality for solvers
pub trait Grid: Resource {
    fn get_x_bounds() {}
    fn get_y_bounds() {}
    fn get_z_bounds() {}
}

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct CartesianGrid {
    // Bounds in 3d space
    #[uniform(0)]
    x_bounds: Vec2,
    #[uniform(0)]
    y_bounds: Vec2,
    #[uniform(0)]
    z_bounds: Vec2,

    // Number of cell divisions per axis
    #[uniform(0)]
    x_divisions: u32,
    #[uniform(0)]
    y_divisions: u32,
    #[uniform(0)]
    z_divisions: u32,

    // Buffers
    #[storage(1)]
    velocity_field: Vec<Vec3>,
    #[storage(2)]
    pressure_field: Vec<f32>,
}
impl CartesianGrid {
    /// Creates a new caretisian grid.
    /// - `width` corresponds to x-axis
    /// - `height` corresponds to y-axis (up-axis)
    /// - `depth` corresponds to z-axis
    pub fn new(
        width: f32,
        height: f32,
        depth: f32,
        x_divisions: u32,
        y_divisions: u32,
        z_divisions: u32,
    ) -> Self {
        let x_bounds = Vec2 {
            x: -width / 2.0,
            y: width / 2.0,
        };
        let y_bounds = Vec2 {
            x: -height / 2.0,
            y: height / 2.0,
        };
        let z_bounds = Vec2 {
            x: -depth / 2.0,
            y: depth / 2.0,
        };

        let num_cells = x_divisions * y_divisions * z_divisions;
        let velocity_field = vec![Vec3::ZERO; num_cells as usize];
        let pressure_field = vec![0.0; num_cells as usize];

        Self {
            x_bounds,
            y_bounds,
            z_bounds,
            x_divisions,
            y_divisions,
            z_divisions,
            velocity_field,
            pressure_field,
        }
    }

    fn test_values(
        width: f32,
        height: f32,
        depth: f32,
        x_divisions: u32,
        y_divisions: u32,
        z_divisions: u32,
    ) -> Self {
        let mut grid = Self::new(width, height, depth, x_divisions, y_divisions, z_divisions);

        for y in 0..grid.y_divisions {
            for z in 0..grid.z_divisions {
                let y_component = y as f32 / grid.y_divisions as f32;
                let z_component = z as f32 / grid.z_divisions as f32;

                let i = grid.index(0, y, z);
                grid.pressure_field[i] = z_component + y_component;
            }
        }

        grid
    }

    fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (x * self.y_divisions * self.z_divisions + y * self.z_divisions + z) as usize
    }
}
impl Grid for CartesianGrid {
    fn get_x_bounds() {}

    fn get_y_bounds() {}

    fn get_z_bounds() {}
}
impl Debug for CartesianGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CartesianGrid")
            .field("x_bounds", &self.x_bounds)
            .field("y_bounds", &self.y_bounds)
            .field("z_bounds", &self.z_bounds)
            .field("x_divisions", &self.x_divisions)
            .field("y_divisions", &self.y_divisions)
            .field("z_divisions", &self.z_divisions)
            .finish()
    }
}
impl Resource for CartesianGrid {}
