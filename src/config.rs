use egui::{Color32, Visuals};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ThemeConfig {
    pub name: String,
    pub dark_mode: bool,

    pub override_text_color: Option<[u8; 4]>,
    pub override_weak_text_color: Option<[u8; 4]>,
    pub override_hyperlink_color: Option<[u8; 4]>,
    pub override_faint_bg_color: Option<[u8; 4]>,
    pub override_extreme_bg_color: Option<[u8; 4]>,
    pub override_code_bg_color: Option<[u8; 4]>,
    pub override_warn_fg_color: Option<[u8; 4]>,
    pub override_error_fg_color: Option<[u8; 4]>,

    pub override_window_fill: Option<[u8; 4]>,
    pub override_window_stroke_color: Option<[u8; 4]>,
    pub override_window_stroke_width: Option<f32>,
    pub override_window_corner_radius: Option<u8>,
    pub override_window_shadow_size: Option<u8>,

    pub override_panel_fill: Option<[u8; 4]>,

    pub override_popup_shadow_size: Option<u8>,

    pub override_selection_bg: Option<[u8; 4]>,
    pub override_selection_stroke_color: Option<[u8; 4]>,
    pub override_selection_stroke_width: Option<f32>,

    pub override_widget_noninteractive_bg_fill: Option<[u8; 4]>,
    pub override_widget_noninteractive_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_noninteractive_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_noninteractive_bg_stroke_width: Option<f32>,
    pub override_widget_noninteractive_corner_radius: Option<u8>,
    pub override_widget_noninteractive_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_noninteractive_fg_stroke_width: Option<f32>,
    pub override_widget_noninteractive_expansion: Option<f32>,

    pub override_widget_inactive_bg_fill: Option<[u8; 4]>,
    pub override_widget_inactive_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_inactive_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_inactive_bg_stroke_width: Option<f32>,
    pub override_widget_inactive_corner_radius: Option<u8>,
    pub override_widget_inactive_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_inactive_fg_stroke_width: Option<f32>,
    pub override_widget_inactive_expansion: Option<f32>,

    pub override_widget_hovered_bg_fill: Option<[u8; 4]>,
    pub override_widget_hovered_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_hovered_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_hovered_bg_stroke_width: Option<f32>,
    pub override_widget_hovered_corner_radius: Option<u8>,
    pub override_widget_hovered_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_hovered_fg_stroke_width: Option<f32>,
    pub override_widget_hovered_expansion: Option<f32>,

    pub override_widget_active_bg_fill: Option<[u8; 4]>,
    pub override_widget_active_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_active_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_active_bg_stroke_width: Option<f32>,
    pub override_widget_active_corner_radius: Option<u8>,
    pub override_widget_active_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_active_fg_stroke_width: Option<f32>,
    pub override_widget_active_expansion: Option<f32>,

    pub override_widget_open_bg_fill: Option<[u8; 4]>,
    pub override_widget_open_weak_bg_fill: Option<[u8; 4]>,
    pub override_widget_open_bg_stroke_color: Option<[u8; 4]>,
    pub override_widget_open_bg_stroke_width: Option<f32>,
    pub override_widget_open_corner_radius: Option<u8>,
    pub override_widget_open_fg_stroke_color: Option<[u8; 4]>,
    pub override_widget_open_fg_stroke_width: Option<f32>,
    pub override_widget_open_expansion: Option<f32>,

    pub override_resize_corner_size: Option<f32>,
    pub override_text_cursor_width: Option<f32>,
    pub override_clip_rect_margin: Option<f32>,
    pub override_button_frame: Option<bool>,
    pub override_collapsing_header_frame: Option<bool>,
    pub override_indent_has_left_vline: Option<bool>,
    pub override_striped: Option<bool>,
    pub override_slider_trailing_fill: Option<bool>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            override_text_color: None,
            override_weak_text_color: None,
            override_hyperlink_color: None,
            override_faint_bg_color: None,
            override_extreme_bg_color: None,
            override_code_bg_color: None,
            override_warn_fg_color: None,
            override_error_fg_color: None,
            override_window_fill: None,
            override_window_stroke_color: None,
            override_window_stroke_width: None,
            override_window_corner_radius: None,
            override_window_shadow_size: None,
            override_panel_fill: None,
            override_popup_shadow_size: None,
            override_selection_bg: None,
            override_selection_stroke_color: None,
            override_selection_stroke_width: None,
            override_widget_noninteractive_bg_fill: None,
            override_widget_noninteractive_weak_bg_fill: None,
            override_widget_noninteractive_bg_stroke_color: None,
            override_widget_noninteractive_bg_stroke_width: None,
            override_widget_noninteractive_corner_radius: None,
            override_widget_noninteractive_fg_stroke_color: None,
            override_widget_noninteractive_fg_stroke_width: None,
            override_widget_noninteractive_expansion: None,
            override_widget_inactive_bg_fill: None,
            override_widget_inactive_weak_bg_fill: None,
            override_widget_inactive_bg_stroke_color: None,
            override_widget_inactive_bg_stroke_width: None,
            override_widget_inactive_corner_radius: None,
            override_widget_inactive_fg_stroke_color: None,
            override_widget_inactive_fg_stroke_width: None,
            override_widget_inactive_expansion: None,
            override_widget_hovered_bg_fill: None,
            override_widget_hovered_weak_bg_fill: None,
            override_widget_hovered_bg_stroke_color: None,
            override_widget_hovered_bg_stroke_width: None,
            override_widget_hovered_corner_radius: None,
            override_widget_hovered_fg_stroke_color: None,
            override_widget_hovered_fg_stroke_width: None,
            override_widget_hovered_expansion: None,
            override_widget_active_bg_fill: None,
            override_widget_active_weak_bg_fill: None,
            override_widget_active_bg_stroke_color: None,
            override_widget_active_bg_stroke_width: None,
            override_widget_active_corner_radius: None,
            override_widget_active_fg_stroke_color: None,
            override_widget_active_fg_stroke_width: None,
            override_widget_active_expansion: None,
            override_widget_open_bg_fill: None,
            override_widget_open_weak_bg_fill: None,
            override_widget_open_bg_stroke_color: None,
            override_widget_open_bg_stroke_width: None,
            override_widget_open_corner_radius: None,
            override_widget_open_fg_stroke_color: None,
            override_widget_open_fg_stroke_width: None,
            override_widget_open_expansion: None,
            override_resize_corner_size: None,
            override_text_cursor_width: None,
            override_clip_rect_margin: None,
            override_button_frame: None,
            override_collapsing_header_frame: None,
            override_indent_has_left_vline: None,
            override_striped: None,
            override_slider_trailing_fill: None,
        }
    }
}

impl ThemeConfig {
    pub fn dark_preset() -> Self {
        Self {
            name: "Dark".to_string(),
            dark_mode: true,
            ..Default::default()
        }
    }

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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
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

        if let Some(color) = self.override_weak_text_color {
            visuals.weak_text_color = Some(Color32::from_rgba_unmultiplied(
                color[0], color[1], color[2], color[3],
            ));
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

        if let Some(color) = self.override_window_fill {
            visuals.window_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_window_stroke_color {
            visuals.window_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_window_stroke_width {
            visuals.window_stroke.width = width;
        }

        if let Some(radius) = self.override_window_corner_radius {
            visuals.window_corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(size) = self.override_window_shadow_size {
            visuals.window_shadow.spread = size;
        }

        if let Some(color) = self.override_panel_fill {
            visuals.panel_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(size) = self.override_popup_shadow_size {
            visuals.popup_shadow.spread = size;
        }

        if let Some(color) = self.override_selection_bg {
            visuals.selection.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_selection_stroke_color {
            visuals.selection.stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_selection_stroke_width {
            visuals.selection.stroke.width = width;
        }

        if let Some(color) = self.override_widget_noninteractive_bg_fill {
            visuals.widgets.noninteractive.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_noninteractive_weak_bg_fill {
            visuals.widgets.noninteractive.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_noninteractive_bg_stroke_color {
            visuals.widgets.noninteractive.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_noninteractive_bg_stroke_width {
            visuals.widgets.noninteractive.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_noninteractive_corner_radius {
            visuals.widgets.noninteractive.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_noninteractive_fg_stroke_color {
            visuals.widgets.noninteractive.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_noninteractive_fg_stroke_width {
            visuals.widgets.noninteractive.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_noninteractive_expansion {
            visuals.widgets.noninteractive.expansion = expansion;
        }

        if let Some(color) = self.override_widget_inactive_bg_fill {
            visuals.widgets.inactive.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_inactive_weak_bg_fill {
            visuals.widgets.inactive.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_inactive_bg_stroke_color {
            visuals.widgets.inactive.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_inactive_bg_stroke_width {
            visuals.widgets.inactive.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_inactive_corner_radius {
            visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_inactive_fg_stroke_color {
            visuals.widgets.inactive.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_inactive_fg_stroke_width {
            visuals.widgets.inactive.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_inactive_expansion {
            visuals.widgets.inactive.expansion = expansion;
        }

        if let Some(color) = self.override_widget_hovered_bg_fill {
            visuals.widgets.hovered.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_hovered_weak_bg_fill {
            visuals.widgets.hovered.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_hovered_bg_stroke_color {
            visuals.widgets.hovered.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_hovered_bg_stroke_width {
            visuals.widgets.hovered.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_hovered_corner_radius {
            visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_hovered_fg_stroke_color {
            visuals.widgets.hovered.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_hovered_fg_stroke_width {
            visuals.widgets.hovered.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_hovered_expansion {
            visuals.widgets.hovered.expansion = expansion;
        }

        if let Some(color) = self.override_widget_active_bg_fill {
            visuals.widgets.active.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_active_weak_bg_fill {
            visuals.widgets.active.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_active_bg_stroke_color {
            visuals.widgets.active.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_active_bg_stroke_width {
            visuals.widgets.active.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_active_corner_radius {
            visuals.widgets.active.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_active_fg_stroke_color {
            visuals.widgets.active.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_active_fg_stroke_width {
            visuals.widgets.active.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_active_expansion {
            visuals.widgets.active.expansion = expansion;
        }

        if let Some(color) = self.override_widget_open_bg_fill {
            visuals.widgets.open.bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_open_weak_bg_fill {
            visuals.widgets.open.weak_bg_fill =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(color) = self.override_widget_open_bg_stroke_color {
            visuals.widgets.open.bg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_open_bg_stroke_width {
            visuals.widgets.open.bg_stroke.width = width;
        }

        if let Some(radius) = self.override_widget_open_corner_radius {
            visuals.widgets.open.corner_radius = egui::CornerRadius::same(radius);
        }

        if let Some(color) = self.override_widget_open_fg_stroke_color {
            visuals.widgets.open.fg_stroke.color =
                Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3]);
        }

        if let Some(width) = self.override_widget_open_fg_stroke_width {
            visuals.widgets.open.fg_stroke.width = width;
        }

        if let Some(expansion) = self.override_widget_open_expansion {
            visuals.widgets.open.expansion = expansion;
        }

        if let Some(size) = self.override_resize_corner_size {
            visuals.resize_corner_size = size;
        }

        if let Some(width) = self.override_text_cursor_width {
            visuals.text_cursor.stroke.width = width;
        }

        if let Some(margin) = self.override_clip_rect_margin {
            visuals.clip_rect_margin = margin;
        }

        if let Some(button_frame) = self.override_button_frame {
            visuals.button_frame = button_frame;
        }

        if let Some(collapsing_header_frame) = self.override_collapsing_header_frame {
            visuals.collapsing_header_frame = collapsing_header_frame;
        }

        if let Some(indent_has_left_vline) = self.override_indent_has_left_vline {
            visuals.indent_has_left_vline = indent_has_left_vline;
        }

        if let Some(striped) = self.override_striped {
            visuals.striped = striped;
        }

        if let Some(slider_trailing_fill) = self.override_slider_trailing_fill {
            visuals.slider_trailing_fill = slider_trailing_fill;
        }

        visuals
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

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
            ..Default::default()
        }
    }
}
