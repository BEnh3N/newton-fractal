use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{shader::Root, ShaderParams};

pub fn update_gui(
    mut contexts: EguiContexts,
    mut params: ResMut<ShaderParams>,
    time: Res<Time>,
    mut _roots: Query<&mut Root>,
) {
    let ctx = contexts.ctx_mut().unwrap();
    egui::Window::new("Newton Fractal").show(ctx, |ui| {
        egui::Grid::new("params_grid").show(ui, |ui| {
            ui.label("FPS");
            ui.monospace(format!("{:.0}", 1.0 / time.delta_secs()));
            ui.end_row();

            ui.label("Max Iter");
            ui.add(egui::DragValue::new(&mut params.max_iterations).range(0..=100));
            ui.end_row();

            ui.label("Scale");
            ui.horizontal(|ui| {
                ui.monospace("x");
                ui.add(egui::DragValue::new(&mut params.scale));
                ui.add_enabled(
                    false,
                    egui::Label::new(
                        egui::RichText::new(format!("{:.2}", params.aspect_ratio)).monospace(),
                    ),
                );
            });
            ui.end_row();

            ui.label("Offset");
            let offset_speed = 1.0 / params.scale / 10.0;
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut params.offset.x).speed(offset_speed));
                ui.monospace("i, ");
                ui.add(egui::DragValue::new(&mut params.offset.y).speed(offset_speed));
                ui.monospace("j");
            });
            ui.end_row();

            ui.label("Epsilon");
            ui.horizontal(|ui| {
                if ui.button("×10").clicked() {
                    params.epsilon *= 10.0;
                    if params.epsilon > 1.0 {
                        params.epsilon = 1.0;
                    }
                }
                ui.monospace(format!("{:.5}", params.epsilon));
                if ui.button("÷10").clicked() {
                    params.epsilon /= 10.0;
                    if params.epsilon < 0.00001 {
                        params.epsilon = 0.00001;
                    }
                }
            });
            // ui.end_row();

            // ui.collapsing("Roots", |ui| {
            //     for mut root in roots.iter_mut() {
            //         ui.horizontal(|ui| {
            //             ui.label(format!("Root ({:.2}, {:.2})", root.pos.x, root.pos.y));
            //             let mut color = [root.color.red, root.color.green, root.color.blue];
            //             if ui.color_edit_button_rgb(&mut color).changed() {
            //                 root.color = LinearRgba::rgb(color[0], color[1], color[2])
            //             }
            //             // ui.color_edit_button_srgba(&mut root.color);
            //             // if ui.button("Reset").clicked() {
            //             //     root.pos = params.offset; // Reset to the current offset
            //             //     root.color = Color::srgb_u8(255, 255, 255).into(); // Reset to default color
            //             // }
            //         });
            //         // ui.end_row();
            //     }
            // })
        })
    });
}
