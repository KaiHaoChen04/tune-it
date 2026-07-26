use eframe::egui::{Context, FontId, TextStyle, FontFamily};

pub fn set_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(30.0, FontFamily::Monospace))
    ].into();
    ctx.set_style(style);
}
