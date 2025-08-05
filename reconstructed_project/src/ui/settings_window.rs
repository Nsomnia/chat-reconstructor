use egui::{Context, Ui};
use vibestream::app::Settings;
use rfd::FileDialog;

pub struct SettingsWindow {
    settings: Settings,
}

impl SettingsWindow {
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    pub fn show(&mut self, ctx: &Context, settings: &mut Settings) -> bool {
        let mut open = true;

        egui::Window::new("Settings")
            .open(&mut open)
            .resizable(true)
            .show(ctx, |ui| {
                self.settings_ui(ui, settings);
            });

        !open
    }

    fn settings_ui(&mut self, ui: &mut Ui, settings: &mut Settings) {
        ui.heading("General Settings");
        ui.separator();

        // Artist name
        ui.horizontal(|ui| {
            ui.label("Artist Name:");
            ui.text_edit_singleline(&mut settings.artist_name);
        });

        // Social media link
        ui.horizontal(|ui| {
            ui.label("Social Media Link:");
            ui.text_edit_singleline(&mut settings.social_media_link);
        });

        ui.add_space(10.0);
        ui.heading("Text Overlay Settings");
        ui.separator();

        // Font path
        ui.horizontal(|ui| {
            ui.label("Font Path:");
            ui.text_edit_singleline(&mut settings.font_path);
            if ui.button("Browse...").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("Font Files", &["ttf", "otf"])
                    .pick_file()
                {
                    settings.font_path = path;
                }
            }
        });

        // Font size
        ui.horizontal(|ui| {
            ui.label("Font Size:");
            ui.add(egui::Slider::new(&mut settings.font_size, 8..=72));
        });

        // Text opacity
        ui.horizontal(|ui| {
            ui.label("Text Opacity:");
            ui.add(egui::Slider::new(&mut settings.text_opacity, 0.0..=1.0));
        });

        // Text color
        ui.horizontal(|ui| {
            ui.label("Text Color:");
            ui.color_edit_button_rgba_unmultiplied(&mut settings.text_color);
        });

        // Text move speed
        ui.horizontal(|ui| {
            ui.label("Text Move Speed:");
            ui.add(egui::Slider::new(&mut settings.text_move_speed, 10.0..=200.0));
        });

        // Fade duration
        ui.horizontal(|ui| {
            ui.label("Fade Duration (seconds):");
            ui.add(egui::Slider::new(&mut settings.fade_duration_secs, 1.0..=30.0));
        });

        ui.add_space(10.0);
        ui.heading("Social Media Settings");
        ui.separator();

        // Social media display duration
        ui.horizontal(|ui| {
            ui.label("Display Duration (seconds):");
            ui.add(egui::Slider::new(&mut settings.social_media_display_duration_secs, 5.0..=60.0));
        });

        // Social media font size
        ui.horizontal(|ui| {
            ui.label("Font Size:");
            ui.add(egui::Slider::new(&mut settings.social_media_font_size, 8..=48));
        });

        // Social media opacity
        ui.horizontal(|ui| {
            ui.label("Opacity:");
            ui.add(egui::Slider::new(&mut settings.social_media_opacity, 0.0..=1.0));
        });

        ui.add_space(10.0);

        // Save button
        if ui.button("Save").clicked() {
            // In a real implementation, we would save the settings
            // For now, we'll just print a message
            println!("Settings saved");
        }
    }
}
