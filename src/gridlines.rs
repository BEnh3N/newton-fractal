use bevy::prelude::*;

use crate::{coordinate_to_screen_space, ShaderParams};

pub fn create_gridlines(
    mut commands: Commands,
    window: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window.single().unwrap();

    // Spawn in gridlines
    commands.spawn((
        HorizontalGridline,
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_scale(Vec3::new(window.width(), 1.0, 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.5)),
    ));
    commands.spawn((
        VerticalGridline,
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::from_scale(Vec3::new(1.0, window.height(), 1.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.5)),
    ));
}

pub fn update_gridlines(
    window: Query<&Window>,
    params: Res<ShaderParams>,
    mut h_gridlines: Query<&mut Transform, (With<HorizontalGridline>, Without<VerticalGridline>)>,
    mut v_gridlines: Query<&mut Transform, (With<VerticalGridline>, Without<HorizontalGridline>)>,
) {
    let window = window.single().unwrap();
    let screen_pos = coordinate_to_screen_space(Vec2::ZERO, window, &params);

    for mut h_gridline in h_gridlines.iter_mut() {
        h_gridline.scale = Vec3::new(window.width(), 1.0, 1.0);
        h_gridline.translation = Vec3::new(0.0, screen_pos.y, 0.5);
    }
    for mut v_gridline in v_gridlines.iter_mut() {
        v_gridline.scale = Vec3::new(1.0, window.height(), 1.0);
        v_gridline.translation = Vec3::new(screen_pos.x, 0.0, 0.5);
    }
}

#[derive(Component)]
pub struct VerticalGridline;

#[derive(Component)]
pub struct HorizontalGridline;
