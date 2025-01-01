use bevy::{prelude::*, window::WindowResized};
use shader::*;

pub mod complex_math;
pub mod drag_and_drop;
pub mod shader;

pub fn window_resize(
    mut resize: EventReader<WindowResized>,
    params: Res<ShaderParams>,
    mut shader: Query<&mut Transform, With<ShaderEntity>>,
    mut roots: Query<(&Root, &mut Transform), Without<ShaderEntity>>,
) {
    if let Some(e) = resize.read().last() {
        let mut shader_transform = shader.get_single_mut().unwrap();
        shader_transform.scale = Vec3::new(e.width, e.height, 1.0);

        for (root, mut root_transform) in roots.iter_mut() {
            root_transform.translation.x = root.pos.x / params.scale.x * e.width / 2.0;
            root_transform.translation.y = root.pos.y / params.scale.y * e.height / 2.0;
        }
    }
}
