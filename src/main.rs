use eframe::egui::CentralPanel;
use eframe::NativeOptions;

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tune It");
        });
    }
}

fn main() -> eframe::Result<()> {
    println!("Hello, world!");

    let options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Tune it",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)),
    )
}
