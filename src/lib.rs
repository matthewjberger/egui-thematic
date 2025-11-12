//! # egui-thematic
//!
//! A comprehensive theme editor and configuration system for [egui](https://github.com/emilk/egui) applications
//! with live preview, preset management, random theme generation, and persistence.
//!
//! ## Features
//!
//! - **Full Theme Configuration**: Customize all visual aspects of your egui application
//! - **Built-in Presets**: Dark and Light themes included out of the box
//! - **Random Theme Generation**: Generate completely random themes with a single click
//! - **Live Preview**: See changes in real-time as you edit
//! - **Persistence**: Save and load themes to/from JSON files
//! - **Interactive Theme Editor**: Full-featured UI with color pickers and preview panel
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use egui_thematic::{ThemeConfig, ThemeEditorState, render_theme_editor};
//!
//! struct MyApp {
//!     theme_editor_state: ThemeEditorState,
//! }
//!
//! impl eframe::App for MyApp {
//!     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//!         // Apply the current theme
//!         let visuals = self.theme_editor_state.current_config.to_visuals();
//!         ctx.set_visuals(visuals);
//!
//!         // Render the theme editor
//!         egui::Window::new("Theme Editor")
//!             .show(ctx, |ui| {
//!                 render_theme_editor(ui, &mut self.theme_editor_state);
//!             });
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! ### Using Presets
//!
//! ```rust
//! use egui_thematic::ThemeConfig;
//!
//! let dark_theme = ThemeConfig::dark_preset();
//! let light_theme = ThemeConfig::light_preset();
//! ```
//!
//! ### Generating Random Themes
//!
//! ```rust
//! use egui_thematic::ThemeConfig;
//!
//! let random_theme = ThemeConfig::randomize();
//! ```
//!
//! ### Saving and Loading
//!
//! ```rust,no_run
//! # use egui_thematic::ThemeConfig;
//! # use std::path::Path;
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let theme = ThemeConfig::dark_preset();
//! theme.save_to_file(Path::new("my_theme.theme.json"))?;
//!
//! let loaded = ThemeConfig::load_from_file(Path::new("my_theme.theme.json"))?;
//! # Ok(())
//! # }
//! ```

use bevy::prelude::Resource;
use egui::{Color32, Visuals};

/// Configuration for an egui theme.
///
/// This structure holds all the color and visual settings that can be customized.
/// All color fields are optional - when `None`, egui's default values for the
/// selected mode (dark/light) are used.
///
/// # Example
///
/// ```rust
/// use egui_thematic::ThemeConfig;
///
/// // Create a dark theme
/// let theme = ThemeConfig::dark_preset();
///
/// // Convert to egui Visuals and apply
/// let visuals = theme.to_visuals();
/// // ctx.set_visuals(visuals);
/// ```
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ThemeConfig {
    pub name: String,
    pub dark_mode: bool,

    pub override_text_color: Option<[u8; 4]>,
    pub override_window_fill: Option<[u8; 4]>,
    pub override_panel_fill: Option<[u8; 4]>,
    pub override_selection_bg: Option<[u8; 4]>,
    pub override_hyperlink_color: Option<[u8; 4]>,
    pub override_faint_bg_color: Option<[u8; 4]>,
    pub override_extreme_bg_color: Option<[u8; 4]>,
    pub override_code_bg_color: Option<[u8; 4]>,
    pub override_warn_fg_color: Option<[u8; 4]>,
    pub override_error_fg_color: Option<[u8; 4]>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            override_text_color: None,
            override_window_fill: None,
            override_panel_fill: None,
            override_selection_bg: None,
            override_hyperlink_color: None,
            override_faint_bg_color: None,
            override_extreme_bg_color: None,
            override_code_bg_color: None,
            override_warn_fg_color: None,
            override_error_fg_color: None,
        }
    }
}

impl ThemeConfig {
    /// Creates a dark theme preset.
    ///
    /// This returns a theme configuration with dark mode enabled and all color
    /// overrides set to `None`, which will use egui's default dark theme colors.
    pub fn dark_preset() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            ..Default::default()
        }
    }

    /// Creates a light theme preset.
    ///
    /// This returns a theme configuration with dark mode disabled and all color
    /// overrides set to `None`, which will use egui's default light theme colors.
    pub fn light_preset() -> Self {
        Self {
            name: "Light".to_string(),
            dark_mode: false,
            ..Default::default()
        }
    }

    /// Converts this theme configuration to egui's `Visuals` type.
    ///
    /// This applies all configured color overrides to the base dark or light theme.
    /// Any `None` values will use egui's defaults for the selected mode.
    ///
    /// # Example
    ///
    /// ```rust
    /// use egui_thematic::ThemeConfig;
    ///
    /// let theme = ThemeConfig::dark_preset();
    /// let visuals = theme.to_visuals();
    /// // Apply with: ctx.set_visuals(visuals);
    /// ```
    pub fn to_visuals(&self) -> Visuals {
        let mut visuals = if self.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        if let Some(color) = self.override_text_color {
            visuals.override_text_color = Some(Color32::from_rgba_unmultiplied(
                color[0], color[1], color[2], color[3],
            ));
        }

        if let Some(color) = self.override_window_fill {
            visuals.window_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_panel_fill {
            visuals.panel_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_selection_bg {
            visuals.selection.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_hyperlink_color {
            visuals.hyperlink_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_faint_bg_color {
            visuals.faint_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_extreme_bg_color {
            visuals.extreme_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_code_bg_color {
            visuals.code_bg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_warn_fg_color {
            visuals.warn_fg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_error_fg_color {
            visuals.error_fg_color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        visuals
    }

    /// Saves this theme configuration to a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written or the theme cannot be serialized.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use egui_thematic::ThemeConfig;
    /// # use std::path::Path;
    /// # fn main() -> Result<(), std::io::Error> {
    /// let theme = ThemeConfig::dark_preset();
    /// theme.save_to_file(Path::new("my_theme.theme.json"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Loads a theme configuration from a JSON file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use egui_thematic::ThemeConfig;
    /// # use std::path::Path;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let theme = ThemeConfig::load_from_file(Path::new("my_theme.theme.json"))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&json)?;
        Ok(config)
    }

    /// Generates a completely random theme.
    ///
    /// This creates a theme with random colors for all visual elements, including
    /// random dark/light mode selection. All colors use full opacity (alpha = 255).
    ///
    /// This is useful for:
    /// - Quickly exploring different color combinations
    /// - Finding inspiration for custom themes
    /// - Testing UI with extreme color variations
    /// - Having fun with wild color schemes
    ///
    /// # Example
    ///
    /// ```rust
    /// use egui_thematic::ThemeConfig;
    ///
    /// let random_theme = ThemeConfig::randomize();
    /// let visuals = random_theme.to_visuals();
    /// // Apply with: ctx.set_visuals(visuals);
    /// ```
    pub fn randomize() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        let random_color =
            |rng: &mut rand::rngs::ThreadRng| -> [u8; 4] { [rng.gen(), rng.gen(), rng.gen(), 255] };

        let dark_mode = rng.gen_bool(0.5);

        Self {
            name: "Random".to_string(),
            dark_mode,
            override_text_color: Some(random_color(&mut rng)),
            override_window_fill: Some(random_color(&mut rng)),
            override_panel_fill: Some(random_color(&mut rng)),
            override_selection_bg: Some(random_color(&mut rng)),
            override_hyperlink_color: Some(random_color(&mut rng)),
            override_faint_bg_color: Some(random_color(&mut rng)),
            override_extreme_bg_color: Some(random_color(&mut rng)),
            override_code_bg_color: Some(random_color(&mut rng)),
            override_warn_fg_color: Some(random_color(&mut rng)),
            override_error_fg_color: Some(random_color(&mut rng)),
        }
    }
}

/// State for the theme editor UI.
///
/// This resource holds the current theme configuration being edited, available presets,
/// and temporary color values for the color pickers.
///
/// When using with Bevy, initialize it as a resource:
///
/// ```rust,ignore
/// app.init_resource::<ThemeEditorState>();
/// ```
///
/// For standalone egui/eframe apps, store it in your app struct:
///
/// ```rust
/// use egui_thematic::ThemeEditorState;
///
/// struct MyApp {
///     theme_editor_state: ThemeEditorState,
/// }
/// ```
#[derive(Resource)]
pub struct ThemeEditorState {
    /// The currently active theme configuration
    pub current_config: ThemeConfig,
    /// Available theme presets (default includes Dark and Light)
    pub presets: Vec<ThemeConfig>,
    /// Index of the selected preset, or None for custom themes
    pub selected_preset_index: Option<usize>,

    /// Temporary color for the text color picker
    pub temp_text_color: Color32,
    /// Temporary color for the window fill picker
    pub temp_window_fill: Color32,
    /// Temporary color for the panel fill picker
    pub temp_panel_fill: Color32,
    /// Temporary color for the selection background picker
    pub temp_selection_bg: Color32,
    /// Temporary color for the hyperlink color picker
    pub temp_hyperlink_color: Color32,
    /// Temporary color for the faint background picker
    pub temp_faint_bg_color: Color32,
    /// Temporary color for the extreme background picker
    pub temp_extreme_bg_color: Color32,
    /// Temporary color for the code background picker
    pub temp_code_bg_color: Color32,
    /// Temporary color for the warning foreground picker
    pub temp_warn_fg_color: Color32,
    /// Temporary color for the error foreground picker
    pub temp_error_fg_color: Color32,
}

impl Default for ThemeEditorState {
    fn default() -> Self {
        let dark_preset = ThemeConfig::dark_preset();
        let light_preset = ThemeConfig::light_preset();

        let visuals = Visuals::dark();

        Self {
            current_config: dark_preset.clone(),
            presets: vec![dark_preset, light_preset],
            selected_preset_index: Some(0),

            temp_text_color: visuals.text_color(),
            temp_window_fill: visuals.window_fill,
            temp_panel_fill: visuals.panel_fill,
            temp_selection_bg: visuals.selection.bg_fill,
            temp_hyperlink_color: visuals.hyperlink_color,
            temp_faint_bg_color: visuals.faint_bg_color,
            temp_extreme_bg_color: visuals.extreme_bg_color,
            temp_code_bg_color: visuals.code_bg_color,
            temp_warn_fg_color: visuals.warn_fg_color,
            temp_error_fg_color: visuals.error_fg_color,
        }
    }
}

impl ThemeEditorState {
    pub fn reset_temp_colors(&mut self) {
        let visuals = if self.current_config.dark_mode {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        self.temp_text_color = visuals.text_color();
        self.temp_window_fill = visuals.window_fill;
        self.temp_panel_fill = visuals.panel_fill;
        self.temp_selection_bg = visuals.selection.bg_fill;
        self.temp_hyperlink_color = visuals.hyperlink_color;
        self.temp_faint_bg_color = visuals.faint_bg_color;
        self.temp_extreme_bg_color = visuals.extreme_bg_color;
        self.temp_code_bg_color = visuals.code_bg_color;
        self.temp_warn_fg_color = visuals.warn_fg_color;
        self.temp_error_fg_color = visuals.error_fg_color;
    }
}

/// Renders the theme editor UI.
///
/// This function displays a complete theme editor interface including:
/// - Preset selector dropdown
/// - Save/Load theme buttons
/// - Reset to Dark/Light buttons
/// - Randomize theme button
/// - Live preview panel
/// - Color pickers for all theme elements organized in collapsible sections
///
/// Call this function inside an `egui::Window` or other container.
/// The theme is applied immediately as changes are made.
///
/// # Example
///
/// ```rust,ignore
/// egui::Window::new("Theme Editor")
///     .show(ctx, |ui| {
///         render_theme_editor(ui, &mut theme_editor_state);
///     });
/// ```
pub fn render_theme_editor(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("Theme Editor");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Preset:");

        let selected_text = editor_state
            .selected_preset_index
            .and_then(|index| editor_state.presets.get(index))
            .map(|preset| preset.name.clone())
            .unwrap_or_else(|| "Custom".to_string());

        let presets_clone = editor_state.presets.clone();

        egui::ComboBox::from_id_salt("theme_preset_combo")
            .selected_text(selected_text)
            .show_ui(ui, |ui| {
                for (index, preset) in presets_clone.iter().enumerate() {
                    if ui
                        .selectable_value(
                            &mut editor_state.selected_preset_index,
                            Some(index),
                            &preset.name,
                        )
                        .clicked()
                    {
                        editor_state.current_config = preset.clone();
                        editor_state.reset_temp_colors();
                        ui.ctx().set_visuals(preset.to_visuals());
                    }
                }
            });

        if ui.button("Save Theme...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Theme", &["theme.json"])
                .set_file_name("custom.theme.json")
                .save_file()
            {
                if let Err(error) = editor_state.current_config.save_to_file(&path) {
                    eprintln!("Failed to save theme: {error}");
                } else {
                    println!("Theme saved to {:?}", path);
                }
            }
        }

        if ui.button("Load Theme...").clicked() {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Theme", &["theme.json"])
                .pick_file()
            {
                match ThemeConfig::load_from_file(&path) {
                    Ok(config) => {
                        editor_state.current_config = config.clone();
                        editor_state.reset_temp_colors();
                        ui.ctx().set_visuals(config.to_visuals());
                        editor_state.selected_preset_index = None;
                        println!("Theme loaded from {:?}", path);
                    }
                    Err(error) => {
                        eprintln!("Failed to load theme: {error}");
                    }
                }
            }
        }

        if ui.button("Reset to Dark").clicked() {
            editor_state.current_config = ThemeConfig::dark_preset();
            editor_state.reset_temp_colors();
            ui.ctx().set_visuals(Visuals::dark());
            editor_state.selected_preset_index = Some(0);
        }

        if ui.button("Reset to Light").clicked() {
            editor_state.current_config = ThemeConfig::light_preset();
            editor_state.reset_temp_colors();
            ui.ctx().set_visuals(Visuals::light());
            editor_state.selected_preset_index = Some(1);
        }

        if ui.button("Randomize Theme").clicked() {
            editor_state.current_config = ThemeConfig::randomize();
            editor_state.reset_temp_colors();
            ui.ctx()
                .set_visuals(editor_state.current_config.to_visuals());
            editor_state.selected_preset_index = None;
        }
    });

    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        let mut changed = false;

        ui.collapsing("Preview", |ui| {
            ui.add_space(4.0);

            egui::Frame::new()
                .fill(ui.visuals().panel_fill)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.label("This is normal text using the current theme");
                    ui.weak("This is weak text");
                    ui.hyperlink_to("This is a hyperlink", "https://example.com");

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        let _ = ui.button("Normal Button");
                        let _ = ui.small_button("Small Button");
                        if ui
                            .button("Disabled")
                            .on_disabled_hover_text("This button is disabled")
                            .clicked()
                        {}
                    });

                    ui.add_space(8.0);

                    let mut checkbox_state = true;
                    ui.checkbox(&mut checkbox_state, "Checkbox example");

                    let mut radio_value = 1;
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut radio_value, 0, "Option 1");
                        ui.radio_value(&mut radio_value, 1, "Option 2");
                    });

                    ui.add_space(8.0);

                    let mut text_buffer = "Editable text field".to_string();
                    ui.text_edit_singleline(&mut text_buffer);

                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.label("Selection example:");
                        let _ = ui.selectable_label(true, "Selected");
                        let _ = ui.selectable_label(false, "Not selected");
                    });

                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new("Warning message").color(ui.visuals().warn_fg_color),
                    );
                    ui.label(
                        egui::RichText::new("Error message").color(ui.visuals().error_fg_color),
                    );

                    ui.add_space(8.0);

                    egui::Frame::new()
                        .fill(ui.visuals().code_bg_color)
                        .inner_margin(4.0)
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("fn main() { println!(\"Code block\"); }")
                                    .monospace(),
                            );
                        });

                    ui.add_space(8.0);

                    egui::Frame::new()
                        .fill(ui.visuals().faint_bg_color)
                        .inner_margin(4.0)
                        .show(ui, |ui| {
                            ui.label("Faint background");
                        });

                    egui::Frame::new()
                        .fill(ui.visuals().extreme_bg_color)
                        .inner_margin(4.0)
                        .show(ui, |ui| {
                            ui.label("Extreme background");
                        });
                });
        });

        ui.collapsing("General", |ui| {
            if ui
                .checkbox(&mut editor_state.current_config.dark_mode, "Dark Mode")
                .changed()
            {
                editor_state.reset_temp_colors();
                changed = true;
            }
        });

        ui.collapsing("Colors", |ui| {
            ui.horizontal(|ui| {
                ui.label("Text Color:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_text_color)
                    .changed()
                {
                    editor_state.current_config.override_text_color =
                        Some(editor_state.temp_text_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_text_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Window Fill:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_window_fill)
                    .changed()
                {
                    editor_state.current_config.override_window_fill =
                        Some(editor_state.temp_window_fill.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_window_fill = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Panel Fill:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_panel_fill)
                    .changed()
                {
                    editor_state.current_config.override_panel_fill =
                        Some(editor_state.temp_panel_fill.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_panel_fill = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Selection Background:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_selection_bg)
                    .changed()
                {
                    editor_state.current_config.override_selection_bg =
                        Some(editor_state.temp_selection_bg.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_selection_bg = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hyperlink Color:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_hyperlink_color)
                    .changed()
                {
                    editor_state.current_config.override_hyperlink_color =
                        Some(editor_state.temp_hyperlink_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_hyperlink_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Faint Background:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_faint_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_faint_bg_color =
                        Some(editor_state.temp_faint_bg_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_faint_bg_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Extreme Background:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_extreme_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_extreme_bg_color =
                        Some(editor_state.temp_extreme_bg_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_extreme_bg_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Code Background:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_code_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_code_bg_color =
                        Some(editor_state.temp_code_bg_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_code_bg_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Warning Foreground:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_warn_fg_color)
                    .changed()
                {
                    editor_state.current_config.override_warn_fg_color =
                        Some(editor_state.temp_warn_fg_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_warn_fg_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Error Foreground:");
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_error_fg_color)
                    .changed()
                {
                    editor_state.current_config.override_error_fg_color =
                        Some(editor_state.temp_error_fg_color.to_array());
                    changed = true;
                }
                if ui.button("Reset").clicked() {
                    editor_state.current_config.override_error_fg_color = None;
                    editor_state.reset_temp_colors();
                    changed = true;
                }
            });
        });

        if changed {
            let visuals = editor_state.current_config.to_visuals();
            ui.ctx().set_visuals(visuals);
            editor_state.selected_preset_index = None;
        }
    });
}
