use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::WindowResized,
};
use shader::*;

pub mod complex_math;
pub mod drag_and_drop;
pub mod gridlines;
pub mod gui;
pub mod shader;

pub fn coordinate_to_screen_space(pos: Vec2, window: &Window, params: &ShaderParams) -> Vec2 {
    return (pos + params.offset) // apply coordinate offset
        * params.scale // apply scale
        / Vec2::new(params.aspect_ratio, 1.0) // properly scale x axis according to aspect ratio
        * window.size()
        / 2.0;
}

pub fn screen_to_coordinate_space(pos: Vec2, window: &Window, params: &ShaderParams) -> Vec2 {
    return (pos * 2.0 / window.size() * Vec2::new(params.aspect_ratio, 1.0) / params.scale)
        - params.offset;
}

pub fn window_resize(
    mut resize: EventReader<WindowResized>,
    mut shader: Query<&mut Transform, With<ShaderEntity>>,
    mut params: ResMut<ShaderParams>,
) {
    if let Some(e) = resize.read().last() {
        let mut shader_transform = shader.get_single_mut().unwrap();
        shader_transform.scale = Vec3::new(e.width, e.height, 1.0);
        params.aspect_ratio = e.width / e.height;
    }
}

pub fn scroll(mut mouse_scroll: EventReader<MouseWheel>, mut params: ResMut<ShaderParams>) {
    for e in mouse_scroll.read() {
        match e.unit {
            MouseScrollUnit::Line => params.scale *= 1.15_f32.powf(e.y),
            MouseScrollUnit::Pixel => params.scale *= 1.01_f32.powf(e.y),
        }
    }

    if params.scale < 0.5 {
        params.scale = 0.5;
    }
    if params.scale > 10_000_000.0 {
        params.scale = 10_000_000.0;
    }
}

pub fn update_root_pos(
    window: Query<&Window>,
    params: Res<ShaderParams>,
    mut roots: Query<(&Root, &mut Transform)>,
) {
    let window = window.get_single().unwrap();

    for (root, mut root_transform) in roots.iter_mut() {
        root_transform.translation =
            coordinate_to_screen_space(root.pos, window, &params).extend(1.0);
    }
}
