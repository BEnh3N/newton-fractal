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

pub fn coordinate_to_screen_space(
    pos: Vec2,
    window: &Window,
    scale: f32,
    aspect_ratio: f32,
) -> Vec2 {
    return pos * scale / Vec2::new(aspect_ratio, 1.0) * Vec2::new(window.width(), window.height())
        / 2.0;
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
            MouseScrollUnit::Line => {
                if e.y > 0.0 {
                    params.scale *= 1.15;
                } else {
                    params.scale /= 1.15;
                }
            }
            MouseScrollUnit::Pixel => {
                if e.y > 0.0 {
                    for _ in 0..e.y as usize {
                        params.scale *= 1.01;
                    }
                } else {
                    for _ in 0..e.y.abs() as usize {
                        params.scale /= 1.01;
                    }
                }
            }
        }
    }

    if params.scale < 0.5 {
        params.scale = 0.5;
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
            coordinate_to_screen_space(root.pos, window, params.scale, params.aspect_ratio)
                .extend(1.0);
    }
}
