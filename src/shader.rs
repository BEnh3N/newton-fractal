use bevy::{
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
        storage::ShaderStorageBuffer,
    },
    sprite::Material2d,
};

use crate::complex_math::{derivative, expand_polynomial};

#[derive(Component)]
pub struct Shader;

#[derive(Component, ShaderType, Clone, Debug)]
pub struct Root {
    pub pos: Vec2,
    pub color: LinearRgba,
}
impl Root {
    pub fn new(r: f32, i: f32, color: LinearRgba) -> Root {
        Root {
            pos: Vec2::new(r, i),
            color,
        }
    }
}

#[derive(ShaderType, Debug, Clone, Resource)]
pub struct ShaderParams {
    pub epsilon: f32,
    pub max_iterations: u32,
    pub scale: Vec2,
}
impl Default for ShaderParams {
    fn default() -> Self {
        ShaderParams {
            epsilon: 0.01,
            max_iterations: 25,
            scale: Vec2::new(3.5, 2.0),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct NewtonShader {
    #[storage(0, read_only)]
    pub roots: Handle<ShaderStorageBuffer>,
    #[storage(1, read_only)]
    pub coefficients: Handle<ShaderStorageBuffer>,
    #[storage(2, read_only)]
    pub derivative: Handle<ShaderStorageBuffer>,
    #[uniform(3)]
    pub params: ShaderParams,
}
impl NewtonShader {
    pub fn new(
        roots: Handle<ShaderStorageBuffer>,
        coefficients: Handle<ShaderStorageBuffer>,
        derivative: Handle<ShaderStorageBuffer>,
        params: ShaderParams,
    ) -> NewtonShader {
        NewtonShader {
            roots,
            coefficients,
            derivative,
            params,
        }
    }
}
impl Material2d for NewtonShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/newton_shader.wgsl".into()
    }
}

pub fn update_shader_inputs(
    roots: Query<&Root>,
    mut materials: ResMut<Assets<NewtonShader>>,
    params: Res<ShaderParams>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let material = materials.iter_mut().last().unwrap().1;

    material.params = params.clone();

    let roots = roots.iter().cloned().collect();
    let coefficients = expand_polynomial(&roots);
    let derivative = derivative(&coefficients);

    let roots_buffer = buffers.get_mut(&material.roots).unwrap();
    roots_buffer.set_data(roots);
    let coefficients_buffer = buffers.get_mut(&material.coefficients).unwrap();
    coefficients_buffer.set_data(coefficients);
    let derivative_buffer = buffers.get_mut(&material.derivative).unwrap();
    derivative_buffer.set_data(derivative);
}

pub fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut params: ResMut<ShaderParams>) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        params.max_iterations += 1;
        println!("Max iterations: {}", params.max_iterations);
    }

    if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        params.max_iterations -= 1;
        println!("Max iterations: {}", params.max_iterations);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        params.epsilon *= 0.1;
        println!("Epsilon: {}", params.epsilon);
    }
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        params.epsilon *= 10.0;
        println!("Epsilon: {}", params.epsilon);
    }
}
