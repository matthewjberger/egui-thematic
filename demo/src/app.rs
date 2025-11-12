use egui_thematic::{render_theme_editor, ThemeConfig, ThemeEditorState};

pub struct DemoApp {
    theme_editor_state: ThemeEditorState,
    show_theme_editor: bool,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            theme_editor_state: ThemeEditorState::default(),
            show_theme_editor: true,
        }
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let visuals = self.theme_editor_state.current_config.to_visuals();
        ctx.set_visuals(visuals);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("egui-thematic Demo");
                ui.separator();

                if ui.button("Toggle Theme Editor").clicked() {
                    self.show_theme_editor = !self.show_theme_editor;
                }

                ui.separator();

                if ui.button("Dark Preset").clicked() {
                    self.theme_editor_state.current_config = ThemeConfig::dark_preset();
                    self.theme_editor_state.reset_temp_colors();
                    self.theme_editor_state.selected_preset_index = Some(0);
                }

                if ui.button("Light Preset").clicked() {
                    self.theme_editor_state.current_config = ThemeConfig::light_preset();
                    self.theme_editor_state.reset_temp_colors();
                    self.theme_editor_state.selected_preset_index = Some(1);
                }

                if ui.button("Randomize").clicked() {
                    self.theme_editor_state.current_config = ThemeConfig::randomize();
                    self.theme_editor_state.reset_temp_colors();
                    self.theme_editor_state.selected_preset_index = None;
                }
            });
        });

        if self.show_theme_editor {
            egui::Window::new("Theme Editor")
                .open(&mut self.show_theme_editor)
                .resizable(true)
                .default_width(400.0)
                .show(ctx, |ui| {
                    render_theme_editor(ui, &mut self.theme_editor_state);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Welcome to egui-thematic!");
                ui.add_space(8.0);

                ui.label("This demo showcases the egui-thematic crate, which provides a comprehensive theme editor for egui applications.");
                ui.add_space(8.0);

                ui.separator();
                ui.heading("Sample UI Elements");
                ui.add_space(8.0);

                ui.label("This is normal text");
                ui.weak("This is weak text");
                ui.hyperlink_to("This is a hyperlink", "https://github.com");

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let _ = ui.button("Button");
                    let _ = ui.small_button("Small Button");
                });

                ui.add_space(8.0);

                let mut checkbox = true;
                ui.checkbox(&mut checkbox, "Checkbox");

                let mut radio = 0;
                ui.horizontal(|ui| {
                    ui.radio_value(&mut radio, 0, "Option 1");
                    ui.radio_value(&mut radio, 1, "Option 2");
                    ui.radio_value(&mut radio, 2, "Option 3");
                });

                ui.add_space(8.0);

                let mut text = String::from("Editable text");
                ui.text_edit_singleline(&mut text);

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    let _ = ui.selectable_label(true, "Selected");
                    let _ = ui.selectable_label(false, "Not Selected");
                });

                ui.add_space(8.0);

                ui.separator();
                ui.heading("Status Messages");

                ui.label(egui::RichText::new("Warning: This is a warning message").color(ui.visuals().warn_fg_color));
                ui.label(egui::RichText::new("Error: This is an error message").color(ui.visuals().error_fg_color));

                ui.add_space(8.0);

                ui.separator();
                ui.heading("Code Block");

                egui::Frame::new()
                    .fill(ui.visuals().code_bg_color)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("fn main() {\n    println!(\"Hello, world!\");\n}").monospace());
                    });

                ui.add_space(8.0);

                ui.separator();
                ui.heading("Background Examples");

                egui::Frame::new()
                    .fill(ui.visuals().faint_bg_color)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("Faint background");
                    });

                ui.add_space(4.0);

                egui::Frame::new()
                    .fill(ui.visuals().extreme_bg_color)
                    .inner_margin(8.0)
                    .show(ui, |ui| {
                        ui.label("Extreme background");
                    });

                ui.add_space(8.0);

                ui.separator();
                ui.heading("Instructions");
                ui.label("1. Click 'Toggle Theme Editor' to show/hide the theme editor");
                ui.label("2. Use the preset buttons to quickly switch between Dark and Light themes");
                ui.label("3. Open the theme editor to customize individual colors");
                ui.label("4. Use 'Save Theme...' to export your custom theme to a JSON file");
                ui.label("5. Use 'Load Theme...' to import a saved theme");
            });
        });
    }
}
