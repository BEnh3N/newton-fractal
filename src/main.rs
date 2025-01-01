use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::vec2,
    prelude::*,
    render::{
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
        storage::ShaderStorageBuffer,
    },
    sprite::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use num_complex::Complex;

fn main() {
    App::new()
        .init_resource::<DragState>()
        .init_resource::<ShaderParams>()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<NewtonShader>::default(),
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keyboard_input,
                window_resize,
                update_shader_inputs,
                handle_drag,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    window: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<NewtonShader>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    params: Res<ShaderParams>,
) {
    let window = window.get_single().unwrap();

    // Define the roots of the polynomial and calculate the coefficients and derivative
    let roots = vec![
        Root::new(-1.3247, 0.0, Color::srgb_u8(82, 168, 208).into()),
        Root::new(0.0, 1.0, Color::srgb_u8(68, 142, 152).into()),
        Root::new(0.0, -1.0, Color::srgb_u8(96, 152, 92).into()),
        Root::new(0.66236, -0.56228, Color::srgb_u8(62, 83, 130).into()),
        Root::new(0.66236, 0.56228, Color::srgb_u8(75, 15, 94).into()),
    ];
    let coefficients = expand_polynomial(&roots);
    let derivative = derivative(&coefficients);

    for root in &roots {
        let x = root.pos.x / params.scale.x * window.width() / 2.0;
        let y = root.pos.y / params.scale.y * window.height() / 2.0;
        let parent = commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(10.0))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(x, y, 1.0),
                root.clone(),
                Draggable,
            ))
            .id();

        commands.entity(parent).with_child((
            Mesh2d(meshes.add(Circle::new(7.0))),
            MeshMaterial2d(materials.add(Color::from(root.color))),
            Transform::from_xyz(0.0, 0.0, 1.0),
        ));
    }

    // Store the data in their respective buffers
    let roots_handle = buffers.add(ShaderStorageBuffer::from(roots));
    let coefficients_handle = buffers.add(ShaderStorageBuffer::from(coefficients));
    let derivative_handle = buffers.add(ShaderStorageBuffer::from(derivative));

    // Create the material
    let material_handle = custom_materials.add(NewtonShader {
        roots: roots_handle,
        coefficients: coefficients_handle,
        derivative: derivative_handle,
        params: params.clone(),
    });

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(material_handle),
        Transform::from_scale(Vec3::new(window.width(), window.height(), 1.0)),
        Shader,
    ));
    commands.spawn(Camera2d);
}

fn update_shader_inputs(
    roots: Query<&Root>,
    mut materials: ResMut<Assets<NewtonShader>>,
    params: Res<ShaderParams>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
) {
    let material = materials.iter_mut().last().unwrap().1;

    material.params = params.clone();

    let roots: Vec<Root> = roots.iter().cloned().collect();
    let coefficients = expand_polynomial(&roots);
    let derivative = derivative(&coefficients);

    let roots_buffer = buffers.get_mut(&material.roots).unwrap();
    roots_buffer.set_data(roots);
    let coefficients_buffer = buffers.get_mut(&material.coefficients).unwrap();
    coefficients_buffer.set_data(coefficients);
    let derivative_buffer = buffers.get_mut(&material.derivative).unwrap();
    derivative_buffer.set_data(derivative);
}

fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut params: ResMut<ShaderParams>) {
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

fn window_resize(
    mut resize: EventReader<WindowResized>,
    params: Res<ShaderParams>,
    mut shader: Query<&mut Transform, With<Shader>>,
    mut roots: Query<(&Root, &mut Transform), Without<Shader>>,
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

#[derive(Component)]
struct Shader;

#[derive(Component, ShaderType, Clone, Debug)]
struct Root {
    pos: Vec2,
    color: LinearRgba,
}
impl Root {
    fn new(r: f32, i: f32, color: LinearRgba) -> Root {
        Root {
            pos: vec2(r, i),
            color,
        }
    }
}

#[derive(ShaderType, Debug, Clone, Resource)]
struct ShaderParams {
    epsilon: f32,
    max_iterations: u32,
    scale: Vec2,
}

impl Default for ShaderParams {
    fn default() -> Self {
        ShaderParams {
            epsilon: 0.01,
            max_iterations: 25,
            scale: vec2(3.5, 2.0),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct NewtonShader {
    #[storage(0, read_only)]
    roots: Handle<ShaderStorageBuffer>,
    #[storage(1, read_only)]
    coefficients: Handle<ShaderStorageBuffer>,
    #[storage(2, read_only)]
    derivative: Handle<ShaderStorageBuffer>,
    #[uniform(3)]
    params: ShaderParams,
}
impl Material2d for NewtonShader {
    fn fragment_shader() -> ShaderRef {
        "shaders/newton_shader.wgsl".into()
    }
}

fn expand_polynomial(roots: &Vec<Root>) -> Vec<Vec2> {
    let mut coefficients = vec![Complex::new(1.0, 0.0)];

    for root in roots.iter().map(|r| Complex::new(r.pos.x, r.pos.y)) {
        let mut new_coefficients = vec![Complex::new(0.0, 0.0); coefficients.len() + 1];

        for (i, &coef) in coefficients.iter().enumerate() {
            new_coefficients[i] += coef;
            new_coefficients[i + 1] -= coef * root;
        }

        coefficients = new_coefficients;
    }

    coefficients.reverse();
    coefficients.iter().map(|c| vec2(c.re, c.im)).collect()
}

fn derivative(coefficients: &Vec<Vec2>) -> Vec<Vec2> {
    let mut new_coefficients = vec![];

    for (i, coef) in coefficients
        .iter()
        .map(|c| Complex::new(c.x, c.y))
        .enumerate()
        .skip(1)
    {
        new_coefficients.push(coef * i as f32);
    }

    new_coefficients.iter().map(|c| vec2(c.re, c.im)).collect()
}

#[derive(Default, Resource, Debug)]
struct DragState {
    selected_entity: Option<Entity>,
}

#[derive(Component)]
struct Draggable;

fn handle_drag(
    mut drag_state: ResMut<DragState>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    mut draggable_objects: Query<(Entity, &mut Transform, &mut Root), With<Draggable>>,
    params: Res<ShaderParams>,
) {
    let window = windows.get_single().unwrap();
    let cursor_position = if let Some(position) = window.cursor_position() {
        (position - vec2(window.width() / 2.0, window.height() / 2.0)) * vec2(1.0, -1.0)
    } else {
        return;
    };

    // Handle mouse press to select an object
    if mouse_button_input.just_pressed(MouseButton::Left) {
        for (entity, transform, _) in draggable_objects.iter() {
            let position = transform.translation;
            let half_size = 10.0;

            // Check if cursor is over the object
            if cursor_position.x > position.x - half_size
                && cursor_position.x < position.x + half_size
                && cursor_position.y > position.y - half_size
                && cursor_position.y < position.y + half_size
            {
                drag_state.selected_entity = Some(entity);
                break;
            }
        }
    }

    // Handle mouse release to drop the object
    if mouse_button_input.just_released(MouseButton::Left) {
        drag_state.selected_entity = None;
    }

    // Move the selected object
    if let Some(selected_entity) = drag_state.selected_entity {
        if let Ok((_, mut transform, mut root)) = draggable_objects.get_mut(selected_entity) {
            transform.translation = cursor_position.extend(transform.translation.z);
            root.pos =
                cursor_position / vec2(window.width() / 2.0, window.height() / 2.0) * params.scale;
        }
    }
}
