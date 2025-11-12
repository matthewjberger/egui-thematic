use egui_thematic::{render_theme_panel, ThemeEditorState};
use nightshade::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    launch(ThemeEditorDemo::default())?;
    Ok(())
}

#[derive(Default)]
struct ThemeEditorDemo {
    theme_editor_state: ThemeEditorState,
    show_theme_editor: bool,
}

impl State for ThemeEditorDemo {
    fn title(&self) -> &str {
        "egui-thematic Demo"
    }

    fn plugins(&self) -> PluginGroup {
        use nightshade::plugins::PluginGroupExt;
        PluginGroup::new().add(FlyCameraPlugin)
    }

    fn initialize(&mut self, world: &mut World) {
        world.resources.user_interface.enabled = true;
        world.resources.graphics.show_grid = true;
        world.resources.graphics.show_skybox = true;

        let camera_position = Vec3::new(0.0, 2.0, 10.0);
        let main_camera = spawn_camera(world, camera_position, "Main Camera".to_string());
        world.resources.active_camera = Some(main_camera);

        self.show_theme_editor = true;
    }

    fn ui(&mut self, _world: &mut World, ui_context: &egui::Context) {
        render_theme_panel(
            ui_context,
            &mut self.theme_editor_state,
            &mut self.show_theme_editor,
        );

        egui::TopBottomPanel::top("top_panel").show(ui_context, |ui| {
            ui.horizontal(|ui| {
                ui.heading("egui-thematic Demo");
                ui.separator();

                if ui.button("Toggle Theme Editor").clicked() {
                    self.show_theme_editor = !self.show_theme_editor;
                }
            });
        });

        egui::CentralPanel::default().show(ui_context, |ui| {
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
                ui.label("2. Use the preset dropdown to quickly switch between themes");
                ui.label("3. Open the theme editor to customize individual colors");
                ui.label("4. Use 'Save Theme...' to export your custom theme to a JSON file");
                ui.label("5. Use 'Load Theme...' to import a saved theme");
                ui.label("6. Press Q to exit the application");
            });
        });
    }

    fn handle_event(&mut self, _world: &mut World, message: &Message) {
        match message {
            Message::Input { event } => {
                log::debug!("Input event: {:?}", event);
            }
            Message::App { type_name, .. } => {
                log::debug!("App event: {}", type_name);
            }
        }
    }

    fn on_keyboard_input(&mut self, world: &mut World, key_code: KeyCode, key_state: KeyState) {
        if matches!((key_code, key_state), (KeyCode::KeyQ, KeyState::Pressed)) {
            world.resources.window.should_exit = true;
        }
    }
}
