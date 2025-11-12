use crate::config::ThemeConfig;
use egui::{Color32, Visuals};

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
    pub temp_weak_text_color: Color32,
    pub temp_hyperlink_color: Color32,
    pub temp_faint_bg_color: Color32,
    pub temp_extreme_bg_color: Color32,
    pub temp_code_bg_color: Color32,
    pub temp_warn_fg_color: Color32,
    pub temp_error_fg_color: Color32,

    pub temp_window_fill: Color32,
    pub temp_window_stroke_color: Color32,
    pub temp_panel_fill: Color32,
    pub temp_selection_bg: Color32,
    pub temp_selection_stroke_color: Color32,

    pub temp_widget_noninteractive_bg_fill: Color32,
    pub temp_widget_noninteractive_weak_bg_fill: Color32,
    pub temp_widget_noninteractive_bg_stroke_color: Color32,
    pub temp_widget_noninteractive_fg_stroke_color: Color32,

    pub temp_widget_inactive_bg_fill: Color32,
    pub temp_widget_inactive_weak_bg_fill: Color32,
    pub temp_widget_inactive_bg_stroke_color: Color32,
    pub temp_widget_inactive_fg_stroke_color: Color32,

    pub temp_widget_hovered_bg_fill: Color32,
    pub temp_widget_hovered_weak_bg_fill: Color32,
    pub temp_widget_hovered_bg_stroke_color: Color32,
    pub temp_widget_hovered_fg_stroke_color: Color32,

    pub temp_widget_active_bg_fill: Color32,
    pub temp_widget_active_weak_bg_fill: Color32,
    pub temp_widget_active_bg_stroke_color: Color32,
    pub temp_widget_active_fg_stroke_color: Color32,

    pub temp_widget_open_bg_fill: Color32,
    pub temp_widget_open_weak_bg_fill: Color32,
    pub temp_widget_open_bg_stroke_color: Color32,
    pub temp_widget_open_fg_stroke_color: Color32,
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
            temp_weak_text_color: visuals.weak_text_color.unwrap_or(visuals.text_color()),
            temp_hyperlink_color: visuals.hyperlink_color,
            temp_faint_bg_color: visuals.faint_bg_color,
            temp_extreme_bg_color: visuals.extreme_bg_color,
            temp_code_bg_color: visuals.code_bg_color,
            temp_warn_fg_color: visuals.warn_fg_color,
            temp_error_fg_color: visuals.error_fg_color,

            temp_window_fill: visuals.window_fill,
            temp_window_stroke_color: visuals.window_stroke.color,
            temp_panel_fill: visuals.panel_fill,
            temp_selection_bg: visuals.selection.bg_fill,
            temp_selection_stroke_color: visuals.selection.stroke.color,

            temp_widget_noninteractive_bg_fill: visuals.widgets.noninteractive.bg_fill,
            temp_widget_noninteractive_weak_bg_fill: visuals.widgets.noninteractive.weak_bg_fill,
            temp_widget_noninteractive_bg_stroke_color: visuals
                .widgets
                .noninteractive
                .bg_stroke
                .color,
            temp_widget_noninteractive_fg_stroke_color: visuals
                .widgets
                .noninteractive
                .fg_stroke
                .color,

            temp_widget_inactive_bg_fill: visuals.widgets.inactive.bg_fill,
            temp_widget_inactive_weak_bg_fill: visuals.widgets.inactive.weak_bg_fill,
            temp_widget_inactive_bg_stroke_color: visuals.widgets.inactive.bg_stroke.color,
            temp_widget_inactive_fg_stroke_color: visuals.widgets.inactive.fg_stroke.color,

            temp_widget_hovered_bg_fill: visuals.widgets.hovered.bg_fill,
            temp_widget_hovered_weak_bg_fill: visuals.widgets.hovered.weak_bg_fill,
            temp_widget_hovered_bg_stroke_color: visuals.widgets.hovered.bg_stroke.color,
            temp_widget_hovered_fg_stroke_color: visuals.widgets.hovered.fg_stroke.color,

            temp_widget_active_bg_fill: visuals.widgets.active.bg_fill,
            temp_widget_active_weak_bg_fill: visuals.widgets.active.weak_bg_fill,
            temp_widget_active_bg_stroke_color: visuals.widgets.active.bg_stroke.color,
            temp_widget_active_fg_stroke_color: visuals.widgets.active.fg_stroke.color,

            temp_widget_open_bg_fill: visuals.widgets.open.bg_fill,
            temp_widget_open_weak_bg_fill: visuals.widgets.open.weak_bg_fill,
            temp_widget_open_bg_stroke_color: visuals.widgets.open.bg_stroke.color,
            temp_widget_open_fg_stroke_color: visuals.widgets.open.fg_stroke.color,
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
        self.temp_weak_text_color = visuals.weak_text_color.unwrap_or(visuals.text_color());
        self.temp_hyperlink_color = visuals.hyperlink_color;
        self.temp_faint_bg_color = visuals.faint_bg_color;
        self.temp_extreme_bg_color = visuals.extreme_bg_color;
        self.temp_code_bg_color = visuals.code_bg_color;
        self.temp_warn_fg_color = visuals.warn_fg_color;
        self.temp_error_fg_color = visuals.error_fg_color;

        self.temp_window_fill = visuals.window_fill;
        self.temp_window_stroke_color = visuals.window_stroke.color;
        self.temp_panel_fill = visuals.panel_fill;
        self.temp_selection_bg = visuals.selection.bg_fill;
        self.temp_selection_stroke_color = visuals.selection.stroke.color;

        self.temp_widget_noninteractive_bg_fill = visuals.widgets.noninteractive.bg_fill;
        self.temp_widget_noninteractive_weak_bg_fill = visuals.widgets.noninteractive.weak_bg_fill;
        self.temp_widget_noninteractive_bg_stroke_color =
            visuals.widgets.noninteractive.bg_stroke.color;
        self.temp_widget_noninteractive_fg_stroke_color =
            visuals.widgets.noninteractive.fg_stroke.color;

        self.temp_widget_inactive_bg_fill = visuals.widgets.inactive.bg_fill;
        self.temp_widget_inactive_weak_bg_fill = visuals.widgets.inactive.weak_bg_fill;
        self.temp_widget_inactive_bg_stroke_color = visuals.widgets.inactive.bg_stroke.color;
        self.temp_widget_inactive_fg_stroke_color = visuals.widgets.inactive.fg_stroke.color;

        self.temp_widget_hovered_bg_fill = visuals.widgets.hovered.bg_fill;
        self.temp_widget_hovered_weak_bg_fill = visuals.widgets.hovered.weak_bg_fill;
        self.temp_widget_hovered_bg_stroke_color = visuals.widgets.hovered.bg_stroke.color;
        self.temp_widget_hovered_fg_stroke_color = visuals.widgets.hovered.fg_stroke.color;

        self.temp_widget_active_bg_fill = visuals.widgets.active.bg_fill;
        self.temp_widget_active_weak_bg_fill = visuals.widgets.active.weak_bg_fill;
        self.temp_widget_active_bg_stroke_color = visuals.widgets.active.bg_stroke.color;
        self.temp_widget_active_fg_stroke_color = visuals.widgets.active.fg_stroke.color;

        self.temp_widget_open_bg_fill = visuals.widgets.open.bg_fill;
        self.temp_widget_open_weak_bg_fill = visuals.widgets.open.weak_bg_fill;
        self.temp_widget_open_bg_stroke_color = visuals.widgets.open.bg_stroke.color;
        self.temp_widget_open_fg_stroke_color = visuals.widgets.open.fg_stroke.color;
    }
}
