use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::storage::ShaderStorageBuffer,
    sprite::Material2dPlugin,
    window::WindowResized,
};
use newton_fractal::{
    complex_math::{derivative, expand_polynomial},
    drag_and_drop::{handle_drag, DragState, Draggable},
    shader::{keyboard_input, update_shader_inputs, NewtonShader, Root, Shader, ShaderParams},
};

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
    params: Res<ShaderParams>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut custom_materials: ResMut<Assets<NewtonShader>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Query<&Window>,
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
    let material_handle = custom_materials.add(NewtonShader::new(
        roots_handle,
        coefficients_handle,
        derivative_handle,
        params.clone(),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
        MeshMaterial2d(material_handle),
        Transform::from_scale(Vec3::new(window.width(), window.height(), 1.0)),
        Shader,
    ));
    commands.spawn(Camera2d);
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
