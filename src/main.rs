mod encoding;
mod utils;
mod scramble;

use crate::encoding::*;
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use utils::draw_grid;

fn main() {
    nannou::app(model).update(update).run();
}

pub struct Model {
    ui: Egui,
    binary_stream: String,
    encoding: Encodings,
    scrambling: bool,
    scrambling_type: Scramblings,
}

fn model(app: &App) -> Model {
    let main_window = app
        .new_window()
        .title(app.exe_name().unwrap())
        .view(view)
        .size(800, 800)
        .resizable(false)
        .raw_event(raw_ui_event)
        .build()
        .unwrap();

    let window_id = app.window(main_window).unwrap();
    let ui = Egui::from_window(&window_id);

    Model {
        ui,
        binary_stream: "0".to_string(),
        encoding: Encodings::NRZL,
        scrambling: false,
        scrambling_type: Scramblings::B8ZS,
    }
}

fn raw_ui_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.ui.handle_raw_event(event);
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.ui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Simulator Control Panel")
        .collapsible(false)
        .show(&ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("Binary Message:");
                ui.add_space(5.0);
                ui.text_edit_singleline(&mut model.binary_stream);
            });

            let current_encoding = model.encoding;
            ui.vertical(|ui| {
                ui.label("Encoding:");
                ui.add_space(5.0);
                egui::ComboBox::from_label("")
                    .selected_text(format!("{current_encoding:?}"))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut model.encoding, Encodings::NRZL, "NRZ-L");
                        ui.selectable_value(&mut model.encoding, Encodings::NRZI, "NRZ-I");
                        ui.selectable_value(
                            &mut model.encoding,
                            Encodings::Manchester,
                            "Manchester",
                        );
                        ui.selectable_value(
                            &mut model.encoding,
                            Encodings::ManchesterDifferential,
                            "Differential Manchester",
                        );
                        ui.selectable_value(&mut model.encoding, Encodings::AMI, "AMI");
                    });
            });

            if model.encoding == Encodings::AMI {
                ui.horizontal(|ui| {
                    ui.label("Scrambling:");
                    ui.checkbox(&mut model.scrambling, "");
                });

                if model.scrambling == true {
                    let current_scrambling = model.scrambling_type.clone();
                    ui.vertical(|ui| {
                        ui.label("Scrambling:");
                        ui.add_space(5.0);
                        egui::ComboBox::from_label(" ")
                            .selected_text(format!("{current_scrambling:?}"))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut model.scrambling_type,
                                    Scramblings::B8ZS,
                                    "B8ZS",
                                );
                                ui.selectable_value(
                                    &mut model.scrambling_type,
                                    Scramblings::HDB3,
                                    "HDB3",
                                );
                            });
                    });
                }
            }
        });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);

    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);

    match model.encoding {
        Encodings::NRZI => NRZI.encode(&model, &win, &draw),
        Encodings::NRZL => NRZL.encode(&model, &win, &draw),
        Encodings::Manchester => Manchester.encode(&model, &win, &draw),
        Encodings::ManchesterDifferential => {
            ManchesterDifferential.encode(&model, &win, &draw)
        }
        Encodings::AMI => AMI.encode(&model, &win, &draw),
    }

    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(&frame).unwrap();
}
