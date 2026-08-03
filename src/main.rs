use eframe::egui::{self, CentralPanel, TopBottomPanel};
use eframe::NativeOptions;

mod audio;
mod algo;

#[derive(Default)]
struct MyApp {}


impl eframe::App for MyApp {
   fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
       set_style(ctx); 
       show_top_bar(ctx);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tune It");
        });
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
        Box::new(|_cc| Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)),
    )
}
pub fn set_style(ctx: &eframe::egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::new(30.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Button, egui::FontId::new(22.0, egui::FontFamily::Monospace)),
        (egui::TextStyle::Small, egui::FontId::new(14.0, egui::FontFamily::Monospace)),
    ].into();
    ctx.set_style(style);
}
