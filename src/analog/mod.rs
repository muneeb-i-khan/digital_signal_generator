pub mod modulation;

use super::Modulation;
use crate::utils::AnalogSettings;
use nannou_egui::egui;

#[derive(PartialEq)]
pub enum AnalogSignal {
    Sine,
    SawTooth,
}

pub fn draw_ui(ui: &mut egui::Ui, settings: &mut AnalogSettings) {
    ui.horizontal(|ui| {
        ui.radio_value(&mut settings.analog_signal, AnalogSignal::Sine, "Sin(x)");
        ui.radio_value(
            &mut settings.analog_signal,
            AnalogSignal::SawTooth,
            "Saw Tooth",
        );
    });

    ui.add_space(5.0);
    let current_modulation = settings.modulation;
    egui::ComboBox::from_label("   ")
        .selected_text(format!("{current_modulation:?}"))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut settings.modulation, Modulation::PCM, "PCM");
            ui.selectable_value(&mut settings.modulation, Modulation::DM, "DM");
        });

    ui.add(egui::Slider::new(&mut settings.amplitude, -400.0..=400.0).text("Amplitude"));
    ui.add(egui::Slider::new(&mut settings.frequency, 0.001..=0.030).text("Frequency"));
    ui.add(egui::Slider::new(&mut settings.delta, 1.0..=100.0).text("Delta"));
    ui.add(egui::Slider::new(&mut settings.sampling_rate, 0.01..=3.00).text("Sampling Rate"));
}
