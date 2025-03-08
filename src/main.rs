use eframe::egui::{self, FontDefinitions, FontFamily, FontId, Style, TextStyle};

struct MyApp {
    input_text: String,
    output_text: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();
        
        // Ensure egui's built-in monospace font is added
        fonts.families.entry(FontFamily::Monospace).or_default().push("Hack".to_owned()); // Default monospace font

        cc.egui_ctx.set_fonts(fonts);

        // Set up text styles
        let mut style: Style = (*cc.egui_ctx.style()).clone();
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(16.0, FontFamily::Monospace),
        );
        cc.egui_ctx.set_style(style);

        Self {
            input_text: String::new(),
            output_text: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Enter some text:");
            let response = ui.text_edit_singleline(&mut self.input_text);

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.output_text = self.input_text.clone();
                self.input_text.clear();
            }

            ui.separator();
            ui.label("You entered:");
            ui.monospace(&self.output_text);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Echo App",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
    .unwrap();
}
