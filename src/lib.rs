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

    pub fn dracula_preset() -> Self {
        Self {
            name: "Dracula".to_string(),
            dark_mode: true,
            override_text_color: Some([248, 248, 242, 255]),
            override_window_fill: Some([40, 42, 54, 255]),
            override_panel_fill: Some([68, 71, 90, 255]),
            override_selection_bg: Some([98, 114, 164, 255]),
            override_hyperlink_color: Some([139, 233, 253, 255]),
            override_faint_bg_color: Some([68, 71, 90, 255]),
            override_extreme_bg_color: Some([21, 22, 30, 255]),
            override_code_bg_color: Some([68, 71, 90, 255]),
            override_warn_fg_color: Some([241, 250, 140, 255]),
            override_error_fg_color: Some([255, 85, 85, 255]),
        }
    }

    pub fn nord_preset() -> Self {
        Self {
            name: "Nord".to_string(),
            dark_mode: true,
            override_text_color: Some([216, 222, 233, 255]),
            override_window_fill: Some([46, 52, 64, 255]),
            override_panel_fill: Some([59, 66, 82, 255]),
            override_selection_bg: Some([136, 192, 208, 255]),
            override_hyperlink_color: Some([136, 192, 208, 255]),
            override_faint_bg_color: Some([59, 66, 82, 255]),
            override_extreme_bg_color: Some([29, 33, 42, 255]),
            override_code_bg_color: Some([59, 66, 82, 255]),
            override_warn_fg_color: Some([235, 203, 139, 255]),
            override_error_fg_color: Some([191, 97, 106, 255]),
        }
    }

    pub fn gruvbox_dark_preset() -> Self {
        Self {
            name: "Gruvbox Dark".to_string(),
            dark_mode: true,
            override_text_color: Some([235, 219, 178, 255]),
            override_window_fill: Some([40, 40, 40, 255]),
            override_panel_fill: Some([60, 56, 54, 255]),
            override_selection_bg: Some([102, 92, 84, 255]),
            override_hyperlink_color: Some([131, 165, 152, 255]),
            override_faint_bg_color: Some([60, 56, 54, 255]),
            override_extreme_bg_color: Some([29, 32, 33, 255]),
            override_code_bg_color: Some([60, 56, 54, 255]),
            override_warn_fg_color: Some([250, 189, 47, 255]),
            override_error_fg_color: Some([251, 73, 52, 255]),
        }
    }

    pub fn solarized_dark_preset() -> Self {
        Self {
            name: "Solarized Dark".to_string(),
            dark_mode: true,
            override_text_color: Some([131, 148, 150, 255]),
            override_window_fill: Some([0, 43, 54, 255]),
            override_panel_fill: Some([7, 54, 66, 255]),
            override_selection_bg: Some([88, 110, 117, 255]),
            override_hyperlink_color: Some([42, 161, 152, 255]),
            override_faint_bg_color: Some([7, 54, 66, 255]),
            override_extreme_bg_color: Some([0, 30, 38, 255]),
            override_code_bg_color: Some([7, 54, 66, 255]),
            override_warn_fg_color: Some([181, 137, 0, 255]),
            override_error_fg_color: Some([220, 50, 47, 255]),
        }
    }

    pub fn solarized_light_preset() -> Self {
        Self {
            name: "Solarized Light".to_string(),
            dark_mode: false,
            override_text_color: Some([101, 123, 131, 255]),
            override_window_fill: Some([253, 246, 227, 255]),
            override_panel_fill: Some([238, 232, 213, 255]),
            override_selection_bg: Some([147, 161, 161, 255]),
            override_hyperlink_color: Some([38, 139, 210, 255]),
            override_faint_bg_color: Some([238, 232, 213, 255]),
            override_extreme_bg_color: Some([253, 246, 227, 255]),
            override_code_bg_color: Some([238, 232, 213, 255]),
            override_warn_fg_color: Some([181, 137, 0, 255]),
            override_error_fg_color: Some([220, 50, 47, 255]),
        }
    }

    pub fn monokai_preset() -> Self {
        Self {
            name: "Monokai".to_string(),
            dark_mode: true,
            override_text_color: Some([248, 248, 242, 255]),
            override_window_fill: Some([39, 40, 34, 255]),
            override_panel_fill: Some([73, 72, 62, 255]),
            override_selection_bg: Some([73, 72, 62, 255]),
            override_hyperlink_color: Some([102, 217, 239, 255]),
            override_faint_bg_color: Some([73, 72, 62, 255]),
            override_extreme_bg_color: Some([30, 31, 25, 255]),
            override_code_bg_color: Some([73, 72, 62, 255]),
            override_warn_fg_color: Some([230, 219, 116, 255]),
            override_error_fg_color: Some([249, 38, 114, 255]),
        }
    }

    pub fn one_dark_preset() -> Self {
        Self {
            name: "One Dark".to_string(),
            dark_mode: true,
            override_text_color: Some([171, 178, 191, 255]),
            override_window_fill: Some([40, 44, 52, 255]),
            override_panel_fill: Some([33, 37, 43, 255]),
            override_selection_bg: Some([61, 66, 77, 255]),
            override_hyperlink_color: Some([97, 175, 239, 255]),
            override_faint_bg_color: Some([33, 37, 43, 255]),
            override_extreme_bg_color: Some([21, 23, 27, 255]),
            override_code_bg_color: Some([33, 37, 43, 255]),
            override_warn_fg_color: Some([229, 192, 123, 255]),
            override_error_fg_color: Some([224, 108, 117, 255]),
        }
    }

    pub fn tokyo_night_preset() -> Self {
        Self {
            name: "Tokyo Night".to_string(),
            dark_mode: true,
            override_text_color: Some([192, 202, 245, 255]),
            override_window_fill: Some([26, 27, 38, 255]),
            override_panel_fill: Some([36, 40, 59, 255]),
            override_selection_bg: Some([56, 62, 90, 255]),
            override_hyperlink_color: Some([122, 162, 247, 255]),
            override_faint_bg_color: Some([36, 40, 59, 255]),
            override_extreme_bg_color: Some([16, 17, 28, 255]),
            override_code_bg_color: Some([36, 40, 59, 255]),
            override_warn_fg_color: Some([224, 175, 104, 255]),
            override_error_fg_color: Some([247, 118, 142, 255]),
        }
    }

    pub fn catppuccin_mocha_preset() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            dark_mode: true,
            override_text_color: Some([205, 214, 244, 255]),
            override_window_fill: Some([30, 30, 46, 255]),
            override_panel_fill: Some([49, 50, 68, 255]),
            override_selection_bg: Some([88, 91, 112, 255]),
            override_hyperlink_color: Some([137, 180, 250, 255]),
            override_faint_bg_color: Some([49, 50, 68, 255]),
            override_extreme_bg_color: Some([17, 17, 27, 255]),
            override_code_bg_color: Some([49, 50, 68, 255]),
            override_warn_fg_color: Some([249, 226, 175, 255]),
            override_error_fg_color: Some([243, 139, 168, 255]),
        }
    }

    pub fn all_presets() -> Vec<Self> {
        vec![
            Self::dark_preset(),
            Self::light_preset(),
            Self::dracula_preset(),
            Self::nord_preset(),
            Self::gruvbox_dark_preset(),
            Self::solarized_dark_preset(),
            Self::solarized_light_preset(),
            Self::monokai_preset(),
            Self::one_dark_preset(),
            Self::tokyo_night_preset(),
            Self::catppuccin_mocha_preset(),
        ]
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

    pub fn to_rust_code(&self) -> String {
        let mut code = String::new();
        code.push_str("fn apply_theme(ctx: &egui::Context) {\n");
        code.push_str(&format!("    let mut visuals = if {} {{\n", self.dark_mode));
        code.push_str("        egui::Visuals::dark()\n");
        code.push_str("    } else {\n");
        code.push_str("        egui::Visuals::light()\n");
        code.push_str("    };\n\n");

        if let Some(color) = self.override_text_color {
            code.push_str(&format!("    visuals.override_text_color = Some(egui::Color32::from_rgba_unmultiplied({}, {}, {}, {}));\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_window_fill {
            code.push_str(&format!("    visuals.window_fill = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_panel_fill {
            code.push_str(&format!(
                "    visuals.panel_fill = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]
            ));
        }

        if let Some(color) = self.override_selection_bg {
            code.push_str(&format!("    visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_hyperlink_color {
            code.push_str(&format!("    visuals.hyperlink_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_faint_bg_color {
            code.push_str(&format!("    visuals.faint_bg_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_extreme_bg_color {
            code.push_str(&format!("    visuals.extreme_bg_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_code_bg_color {
            code.push_str(&format!("    visuals.code_bg_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_warn_fg_color {
            code.push_str(&format!("    visuals.warn_fg_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        if let Some(color) = self.override_error_fg_color {
            code.push_str(&format!("    visuals.error_fg_color = egui::Color32::from_rgba_unmultiplied({}, {}, {}, {});\n",
                color[0], color[1], color[2], color[3]));
        }

        code.push_str("\n    ctx.set_visuals(visuals);\n");
        code.push_str("}\n");
        code
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
/// This holds the current theme configuration being edited, available presets,
/// and temporary color values for the color pickers.
///
/// # Example
///
/// ```rust
/// use egui_thematic::ThemeEditorState;
///
/// struct MyApp {
///     theme_editor_state: ThemeEditorState,
/// }
/// ```
pub struct ThemeEditorState {
    pub current_config: ThemeConfig,
    pub presets: Vec<ThemeConfig>,
    pub selected_preset_index: Option<usize>,
    pub show_code_export: bool,

    pub storybook_checkbox: bool,
    pub storybook_radio: i32,
    pub storybook_slider: f32,
    pub storybook_text: String,
    pub storybook_combo_selected: usize,

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
        let presets = ThemeConfig::all_presets();
        let dark_preset = presets[0].clone();

        let visuals = Visuals::dark();

        Self {
            current_config: dark_preset,
            presets,
            selected_preset_index: Some(0),
            show_code_export: false,

            storybook_checkbox: true,
            storybook_radio: 1,
            storybook_slider: 50.0,
            storybook_text: "Example text".to_string(),
            storybook_combo_selected: 0,

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
pub fn render_theme_panel(
    ctx: &egui::Context,
    editor_state: &mut ThemeEditorState,
    show_theme_editor: &mut bool,
) {
    let visuals = editor_state.current_config.to_visuals();
    ctx.set_visuals(visuals);

    if *show_theme_editor {
        egui::Window::new("Theme Editor")
            .open(show_theme_editor)
            .resizable(true)
            .default_width(400.0)
            .show(ctx, |ui| {
                render_theme_editor(ui, editor_state);
            });
    }
}

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

        #[cfg(not(target_arch = "wasm32"))]
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

        #[cfg(not(target_arch = "wasm32"))]
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

        if ui.button("Export Code").clicked() {
            editor_state.show_code_export = true;
        }
    });

    if editor_state.show_code_export {
        egui::Window::new("Generated Rust Code")
            .open(&mut editor_state.show_code_export)
            .show(ui.ctx(), |ui| {
                ui.label("Copy this code to your application:");
                ui.add_space(4.0);

                let code = editor_state.current_config.to_rust_code();
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        ui.code(&code);
                    });

                ui.add_space(8.0);
                if ui.button("Copy to Clipboard").clicked() {
                    ui.ctx().copy_text(code.clone());
                }
            });
    }

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

        ui.collapsing("Storybook - Interactive Widget Showcase", |ui| {
            ui.add_space(4.0);
            ui.label("Interact with widgets below to see how your theme affects all controls:");
            ui.add_space(8.0);

            egui::Frame::new()
                .fill(ui.visuals().panel_fill)
                .inner_margin(8.0)
                .show(ui, |ui| {
                    ui.heading("Text Variants");
                    ui.label("Normal text");
                    ui.weak("Weak/secondary text");
                    ui.strong("Strong/bold text");
                    ui.monospace("Monospace text");
                    ui.hyperlink_to("Hyperlink", "https://example.com");
                    ui.label(egui::RichText::new("Warning").color(ui.visuals().warn_fg_color));
                    ui.label(egui::RichText::new("Error").color(ui.visuals().error_fg_color));

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Buttons");
                    ui.horizontal_wrapped(|ui| {
                        let _ = ui.button("Normal Button");
                        let _ = ui.small_button("Small");
                        ui.add_enabled(false, egui::Button::new("Disabled"));
                        if ui.button("Click Me").clicked() {}
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Checkboxes & Radio Buttons");
                    ui.checkbox(&mut editor_state.storybook_checkbox, "Interactive checkbox");
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut editor_state.storybook_radio, 0, "Option A");
                        ui.radio_value(&mut editor_state.storybook_radio, 1, "Option B");
                        ui.radio_value(&mut editor_state.storybook_radio, 2, "Option C");
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Sliders & Progress");
                    ui.add(
                        egui::Slider::new(&mut editor_state.storybook_slider, 0.0..=100.0)
                            .text("Value"),
                    );
                    ui.add(
                        egui::ProgressBar::new(editor_state.storybook_slider / 100.0)
                            .show_percentage(),
                    );

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Text Input");
                    ui.text_edit_singleline(&mut editor_state.storybook_text);
                    ui.text_edit_multiline(&mut editor_state.storybook_text);

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("ComboBox & Selectable");
                    let combo_items = ["First", "Second", "Third", "Fourth"];
                    egui::ComboBox::from_label("Dropdown")
                        .selected_text(combo_items[editor_state.storybook_combo_selected])
                        .show_ui(ui, |ui| {
                            for (index, item) in combo_items.iter().enumerate() {
                                ui.selectable_value(
                                    &mut editor_state.storybook_combo_selected,
                                    index,
                                    *item,
                                );
                            }
                        });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label("Selectable labels:");
                        let _ = ui.selectable_label(
                            editor_state.storybook_combo_selected == 0,
                            "Selected",
                        );
                        let _ = ui.selectable_label(
                            editor_state.storybook_combo_selected != 0,
                            "Not Selected",
                        );
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Backgrounds & Frames");
                    egui::Frame::new()
                        .fill(ui.visuals().faint_bg_color)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.label("Faint background color");
                        });

                    ui.add_space(4.0);

                    egui::Frame::new()
                        .fill(ui.visuals().extreme_bg_color)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.label("Extreme background color");
                        });

                    ui.add_space(4.0);

                    egui::Frame::new()
                        .fill(ui.visuals().code_bg_color)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.monospace("Code background color");
                        });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Collapsing Headers");
                    ui.collapsing("Collapsed by default", |ui| {
                        ui.label("Hidden content inside collapsing header");
                    });

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Separators & Spacing");
                    ui.label("Above separator");
                    ui.separator();
                    ui.label("Below separator");

                    ui.add_space(12.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.heading("Grid Layout");
                    egui::Grid::new("storybook_grid")
                        .num_columns(3)
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("Row 1, Col 1");
                            ui.label("Row 1, Col 2");
                            ui.label("Row 1, Col 3");
                            ui.end_row();

                            ui.label("Row 2, Col 1");
                            let _ = ui.button("Button");
                            ui.checkbox(&mut editor_state.storybook_checkbox, "Check");
                            ui.end_row();
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
