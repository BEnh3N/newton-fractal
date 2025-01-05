use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
    window::WindowResized,
};
use shader::*;

pub mod complex_math;
pub mod drag_and_drop;
pub mod gui;
pub mod shader;

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
            MouseScrollUnit::Line => {}
            MouseScrollUnit::Pixel => {
                params.scale += e.y * 0.1;
                if params.scale < 0.5 {
                    params.scale = 0.5;
                }
            }
        }
    }
}

pub fn update_root_pos(
    window: Query<&Window>,
    params: Res<ShaderParams>,
    mut roots: Query<(&Root, &mut Transform)>,
) {
    let window = window.get_single().unwrap();

    for (root, mut root_transform) in roots.iter_mut() {
        root_transform.translation.x =
            root.pos.x * params.scale / params.aspect_ratio * window.width() / 2.0;
        root_transform.translation.y = root.pos.y * params.scale * window.height() / 2.0;
    }
}
