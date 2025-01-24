use bevy::{
    prelude::*, render::storage::ShaderStorageBuffer, sprite::Material2dPlugin, window::PresentMode,
};
use bevy_egui::EguiPlugin;
use gridlines::{create_gridlines, update_gridlines};
use newton_fractal::{complex_math::*, drag_and_drop::*, gui::*, shader::*, *};

fn main() {
    App::new()
        .init_resource::<DragState>()
        .init_resource::<ShaderParams>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Newton Fractal".into(),
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            }),
            Material2dPlugin::<NewtonShader>::default(),
            EguiPlugin,
        ))
        .add_systems(Startup, (setup, create_gridlines))
        .add_systems(
            Update,
            (
                keyboard_input,
                update_shader_inputs,
                handle_drag,
                scroll,
                update_gui,
                window_resize,
                update_root_pos,
                update_gridlines,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut params: ResMut<ShaderParams>,
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut custom_materials: ResMut<Assets<NewtonShader>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Query<&Window>,
) {
    let window = window.get_single().unwrap();
    params.aspect_ratio = window.width() / window.height();

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
        let screen_pos = coordinate_to_screen_space(root.pos, window, &params);
        let parent = commands
            .spawn((
                Mesh2d(meshes.add(Circle::new(10.0))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(screen_pos.x, screen_pos.y, 1.0),
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
        ShaderEntity,
    ));
    commands.spawn(Camera2d);
}
