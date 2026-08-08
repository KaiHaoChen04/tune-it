use crossbeam_channel::{Receiver};
use eframe::NativeOptions;
use eframe::egui::{self, CentralPanel, TopBottomPanel};

mod algo;
mod audio;

use crate::algo::yin_pitch;
use crate::audio::start_audio_capture;
const NOTE_NAMES:[&str; 12] = ["C","C#","D","D#","E","F","F#","G","G#","A","A#","B"];

struct MyApp {
    rx: Receiver<Vec<f32>>,
    _stream: cpal::Stream,
    sample_buffer: Vec<f32>,
    current_note: Option<(String, f32)>,
    input_level: f32,
}
impl MyApp {
    fn new() -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let stream = start_audio_capture(tx);
        Self {
            rx,
            _stream: stream,
            sample_buffer: Vec::new(),
            current_note: None,
            input_level: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(chunk) = self.rx.try_recv() {
            self.input_level = chunk
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0, |acc, next| acc.max(next));
            self.sample_buffer.extend_from_slice(&chunk);
        }
        const WINDOW: usize = 4096;
        if self.sample_buffer.len() > WINDOW {
            let excess = self.sample_buffer.len() - WINDOW;
            self.sample_buffer.drain(0..excess);
        }

        if self.sample_buffer.len() >= WINDOW {
            if let Some(freq) = yin_pitch(&self.sample_buffer, 44100.0) {
                self.current_note = Some(freq_to_note(freq));
            }
        }

        set_style(ctx);
        show_top_bar(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tune It");
            ui.label(format!("Input level: {:.3}", self.input_level));
            ui.add(egui::ProgressBar::new(self.input_level.min(1.0)).show_percentage());
            if let Some((note, freq)) = &self.current_note {
                ui.label(format!("{note} ({freq:+.1} freq)"));
            } else {
                ui.label("Listening for a stable single note...");
            }
        });

        ctx.request_repaint();
    }
}

fn show_top_bar(ctx: &egui::Context) {
    TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("exit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });
}

fn main() -> eframe::Result<()> {
    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([620.0, 440.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tune it",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()) as Box<dyn eframe::App>)),
    )
}
fn set_style(ctx: &eframe::egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(30.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(18.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(22.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(14.0, egui::FontFamily::Monospace),
        ),
    ]
    .into();
    ctx.set_style(style);
}
fn freq_to_note(freq: f32) -> (String, f32){
    let a4 = 440.0;
    let semitones_from_a4 = 12.0 * (freq / a4).log2();
    let rounded = semitones_from_a4.round();

    let note_index = ((rounded as i32 + 9).rem_euclid(12)) as usize;
    let octave = 4 + ((rounded as i32 + 9) as f32 / 12.0).floor() as i32;

    (format!("{}{}", NOTE_NAMES[note_index], octave), freq)
}
