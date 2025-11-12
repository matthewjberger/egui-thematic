use crate::state::ThemeEditorState;

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
    ctx.request_repaint();

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
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading("🎨 Theme Studio");
        ui.add_space(12.0);

        ui.horizontal(|ui| {
            ui.label("Theme:");
            ui.text_edit_singleline(&mut editor_state.current_config.name);

            ui.add_space(20.0);

            if ui
                .checkbox(&mut editor_state.current_config.dark_mode, "Dark Mode")
                .changed()
            {
                editor_state.reset_temp_colors();
            }

            ui.add_space(20.0);

            let selected_text = editor_state
                .selected_preset_index
                .and_then(|index| editor_state.presets.get(index))
                .map(|preset| preset.name.clone())
                .unwrap_or_else(|| "Custom".to_string());

            egui::ComboBox::from_label("Preset")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for (index, preset) in editor_state.presets.clone().iter().enumerate() {
                        if ui
                            .selectable_value(
                                &mut editor_state.selected_preset_index,
                                Some(index),
                                &preset.name,
                            )
                            .clicked()
                        {
                            editor_state.current_config = preset.clone();
                            editor_state.current_config = preset.clone();
                            editor_state.reset_temp_colors();
                        }
                    }
                });

            ui.add_space(20.0);

            if ui.button("📋 Export Code").clicked() {
                editor_state.show_code_export = !editor_state.show_code_export;
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(12.0);

        render_text_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_button_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_text_input_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_slider_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_checkbox_radio_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_dropdown_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_selection_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_window_panel_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_shape_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_special_colors_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_ui_options_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        render_advanced_widget_states_section(ui, editor_state);
        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        if editor_state.show_code_export {
            ui.heading("📋 Export Code");
            ui.add_space(8.0);

            let code = format!(
                r#"use egui_thematic::ThemeConfig;

let theme = {};

ctx.set_visuals(theme.to_visuals());
"#,
                serde_json::to_string_pretty(&editor_state.current_config).unwrap_or_default()
            );

            ui.add(
                egui::TextEdit::multiline(&mut code.as_str())
                    .code_editor()
                    .desired_width(f32::INFINITY)
                    .desired_rows(15),
            );
            ui.add_space(12.0);
        }
    });
}

fn render_text_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("📝 Text");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("Regular text");
                ui.weak("Weak text");
                ui.hyperlink_to("Hyperlink", "https://example.com");
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Text Color:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_text_color = visuals.text_color();
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_text_color)
                    .changed()
                {
                    editor_state.current_config.override_text_color =
                        Some(editor_state.temp_text_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_text_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Weak Text:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_weak_text_color =
                    visuals.weak_text_color.unwrap_or(visuals.text_color());
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_weak_text_color)
                    .changed()
                {
                    editor_state.current_config.override_weak_text_color =
                        Some(editor_state.temp_weak_text_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_weak_text_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Hyperlink:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_hyperlink_color = visuals.hyperlink_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_hyperlink_color)
                    .changed()
                {
                    editor_state.current_config.override_hyperlink_color =
                        Some(editor_state.temp_hyperlink_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_hyperlink_color = None;
                    editor_state.reset_temp_colors();
                }
            });
        });
    });
}

fn render_button_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("🔘 Buttons");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                let _ = ui.button("Button");
                ui.add_space(4.0);
                ui.add_enabled(false, egui::Button::new("Disabled"));
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.collapsing("Inactive (Resting)", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_inactive_bg_fill = visuals.widgets.inactive.bg_fill;
                    if ui
                        .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_bg_fill)
                        .changed()
                    {
                        editor_state.current_config.override_widget_inactive_bg_fill =
                            Some(editor_state.temp_widget_inactive_bg_fill.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state.current_config.override_widget_inactive_bg_fill = None;
                        editor_state.reset_temp_colors();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Border:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_inactive_bg_stroke_color =
                        visuals.widgets.inactive.bg_stroke.color;
                    if ui
                        .color_edit_button_srgba(
                            &mut editor_state.temp_widget_inactive_bg_stroke_color,
                        )
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_inactive_bg_stroke_color =
                            Some(editor_state.temp_widget_inactive_bg_stroke_color.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_inactive_bg_stroke_color = None;
                        editor_state.reset_temp_colors();
                    }
                });
            });

            ui.collapsing("Hovered", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_hovered_bg_fill = visuals.widgets.hovered.bg_fill;
                    if ui
                        .color_edit_button_srgba(&mut editor_state.temp_widget_hovered_bg_fill)
                        .changed()
                    {
                        editor_state.current_config.override_widget_hovered_bg_fill =
                            Some(editor_state.temp_widget_hovered_bg_fill.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state.current_config.override_widget_hovered_bg_fill = None;
                        editor_state.reset_temp_colors();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Border:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_hovered_bg_stroke_color =
                        visuals.widgets.hovered.bg_stroke.color;
                    if ui
                        .color_edit_button_srgba(
                            &mut editor_state.temp_widget_hovered_bg_stroke_color,
                        )
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_hovered_bg_stroke_color =
                            Some(editor_state.temp_widget_hovered_bg_stroke_color.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_hovered_bg_stroke_color = None;
                        editor_state.reset_temp_colors();
                    }
                });
            });

            ui.collapsing("Active (Clicking)", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_active_bg_fill = visuals.widgets.active.bg_fill;
                    if ui
                        .color_edit_button_srgba(&mut editor_state.temp_widget_active_bg_fill)
                        .changed()
                    {
                        editor_state.current_config.override_widget_active_bg_fill =
                            Some(editor_state.temp_widget_active_bg_fill.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state.current_config.override_widget_active_bg_fill = None;
                        editor_state.reset_temp_colors();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Border:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_active_bg_stroke_color =
                        visuals.widgets.active.bg_stroke.color;
                    if ui
                        .color_edit_button_srgba(
                            &mut editor_state.temp_widget_active_bg_stroke_color,
                        )
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_active_bg_stroke_color =
                            Some(editor_state.temp_widget_active_bg_stroke_color.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_active_bg_stroke_color = None;
                        editor_state.reset_temp_colors();
                    }
                });
            });

            ui.collapsing("Disabled", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_noninteractive_bg_fill =
                        visuals.widgets.noninteractive.bg_fill;
                    if ui
                        .color_edit_button_srgba(
                            &mut editor_state.temp_widget_noninteractive_bg_fill,
                        )
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_noninteractive_bg_fill =
                            Some(editor_state.temp_widget_noninteractive_bg_fill.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_noninteractive_bg_fill = None;
                        editor_state.reset_temp_colors();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Border:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_noninteractive_bg_stroke_color =
                        visuals.widgets.noninteractive.bg_stroke.color;
                    if ui
                        .color_edit_button_srgba(
                            &mut editor_state.temp_widget_noninteractive_bg_stroke_color,
                        )
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_noninteractive_bg_stroke_color = Some(
                            editor_state
                                .temp_widget_noninteractive_bg_stroke_color
                                .to_array(),
                        );
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_noninteractive_bg_stroke_color = None;
                        editor_state.reset_temp_colors();
                    }
                });
            });
        });
    });
}

fn render_text_input_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("✏️ Text Input");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.text_edit_singleline(&mut editor_state.storybook_text);
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_bg_fill = visuals.widgets.inactive.bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_bg_fill)
                    .changed()
                {
                    editor_state.current_config.override_widget_inactive_bg_fill =
                        Some(editor_state.temp_widget_inactive_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_widget_inactive_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Border:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_bg_stroke_color =
                    visuals.widgets.inactive.bg_stroke.color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_bg_stroke_color)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_inactive_bg_stroke_color =
                        Some(editor_state.temp_widget_inactive_bg_stroke_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_inactive_bg_stroke_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Cursor Width:");
                let mut width = editor_state
                    .current_config
                    .override_text_cursor_width
                    .unwrap_or(2.0);
                if ui.add(egui::Slider::new(&mut width, 1.0..=5.0)).changed() {
                    editor_state.current_config.override_text_cursor_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_text_cursor_width = None;
                }
            });
        });
    });
}

fn render_slider_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("🎚️ Sliders");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.add(
                    egui::Slider::new(&mut editor_state.storybook_slider, 0.0..=100.0)
                        .text("Value"),
                );
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_bg_fill = visuals.widgets.inactive.bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_bg_fill)
                    .changed()
                {
                    editor_state.current_config.override_widget_inactive_bg_fill =
                        Some(editor_state.temp_widget_inactive_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_widget_inactive_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                let mut slider_trailing_fill = editor_state
                    .current_config
                    .override_slider_trailing_fill
                    .unwrap_or(false);
                if ui
                    .checkbox(&mut slider_trailing_fill, "Trailing Fill")
                    .changed()
                {
                    editor_state.current_config.override_slider_trailing_fill =
                        Some(slider_trailing_fill);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_slider_trailing_fill = None;
                }
            });
        });
    });
}

fn render_checkbox_radio_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("☑️ Checkbox & Radio");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.checkbox(&mut editor_state.storybook_checkbox, "Checkbox");
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.radio_value(&mut editor_state.storybook_radio, 0, "Option A");
                    ui.radio_value(&mut editor_state.storybook_radio, 1, "Option B");
                });
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_bg_fill = visuals.widgets.inactive.bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_bg_fill)
                    .changed()
                {
                    editor_state.current_config.override_widget_inactive_bg_fill =
                        Some(editor_state.temp_widget_inactive_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_widget_inactive_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Checkmark:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_fg_stroke_color =
                    visuals.widgets.inactive.fg_stroke.color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_fg_stroke_color)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_inactive_fg_stroke_color =
                        Some(editor_state.temp_widget_inactive_fg_stroke_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_inactive_fg_stroke_color = None;
                    editor_state.reset_temp_colors();
                }
            });
        });
    });
}

fn render_dropdown_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("📋 Dropdowns");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                egui::ComboBox::from_label("ComboBox")
                    .selected_text(format!(
                        "Item {}",
                        editor_state.storybook_combo_selected + 1
                    ))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut editor_state.storybook_combo_selected,
                            0,
                            "Item 1",
                        );
                        ui.selectable_value(
                            &mut editor_state.storybook_combo_selected,
                            1,
                            "Item 2",
                        );
                        ui.selectable_value(
                            &mut editor_state.storybook_combo_selected,
                            2,
                            "Item 3",
                        );
                    });
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.collapsing("Open State", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_open_bg_fill = visuals.widgets.open.bg_fill;
                    if ui
                        .color_edit_button_srgba(&mut editor_state.temp_widget_open_bg_fill)
                        .changed()
                    {
                        editor_state.current_config.override_widget_open_bg_fill =
                            Some(editor_state.temp_widget_open_bg_fill.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state.current_config.override_widget_open_bg_fill = None;
                        editor_state.reset_temp_colors();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Border:");
                    let visuals = editor_state.current_config.to_visuals();
                    editor_state.temp_widget_open_bg_stroke_color =
                        visuals.widgets.open.bg_stroke.color;
                    if ui
                        .color_edit_button_srgba(&mut editor_state.temp_widget_open_bg_stroke_color)
                        .changed()
                    {
                        editor_state
                            .current_config
                            .override_widget_open_bg_stroke_color =
                            Some(editor_state.temp_widget_open_bg_stroke_color.to_array());
                    }
                    if ui.small_button("Reset").clicked() {
                        editor_state
                            .current_config
                            .override_widget_open_bg_stroke_color = None;
                        editor_state.reset_temp_colors();
                    }
                });
            });
        });
    });
}

fn render_selection_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("🖱️ Selection");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.horizontal(|ui| {
                    let _ = ui.selectable_label(true, "Selected");
                    let _ = ui.selectable_label(false, "Unselected");
                });
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_selection_bg = visuals.selection.bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_selection_bg)
                    .changed()
                {
                    editor_state.current_config.override_selection_bg =
                        Some(editor_state.temp_selection_bg.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_selection_bg = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Border Color:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_selection_stroke_color = visuals.selection.stroke.color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_selection_stroke_color)
                    .changed()
                {
                    editor_state.current_config.override_selection_stroke_color =
                        Some(editor_state.temp_selection_stroke_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_selection_stroke_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Border Width:");
                let mut width = editor_state
                    .current_config
                    .override_selection_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state.current_config.override_selection_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_selection_stroke_width = None;
                }
            });
        });
    });
}

fn render_window_panel_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("🪟 Windows & Panels");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("This is inside a group");
                ui.label("(mimics window/panel)");
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Window Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_window_fill = visuals.window_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_window_fill)
                    .changed()
                {
                    editor_state.current_config.override_window_fill =
                        Some(editor_state.temp_window_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_window_fill = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Panel Fill:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_panel_fill = visuals.panel_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_panel_fill)
                    .changed()
                {
                    editor_state.current_config.override_panel_fill =
                        Some(editor_state.temp_panel_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_panel_fill = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Border:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_window_stroke_color = visuals.window_stroke.color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_window_stroke_color)
                    .changed()
                {
                    editor_state.current_config.override_window_stroke_color =
                        Some(editor_state.temp_window_stroke_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_window_stroke_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Shadow Size:");
                let shadow = editor_state
                    .current_config
                    .override_window_shadow_size
                    .unwrap_or(16);
                let mut shadow_f32 = shadow as f32;
                if ui
                    .add(egui::Slider::new(&mut shadow_f32, 0.0..=50.0))
                    .changed()
                {
                    editor_state.current_config.override_window_shadow_size =
                        Some(shadow_f32 as u8);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_window_shadow_size = None;
                }
            });
        });
    });
}

fn render_shape_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("⬜ Shape & Borders");
    ui.add_space(8.0);

    ui.strong("Global Properties:");
    ui.add_space(4.0);

    ui.horizontal(|ui| {
        ui.label("Corner Radius:");
        let radius = editor_state
            .current_config
            .override_widget_inactive_corner_radius
            .unwrap_or(2);
        let mut radius_f32 = radius as f32;
        if ui
            .add(egui::Slider::new(&mut radius_f32, 0.0..=20.0).text("px"))
            .changed()
        {
            let new_radius = radius_f32 as u8;
            editor_state
                .current_config
                .override_widget_inactive_corner_radius = Some(new_radius);
            editor_state
                .current_config
                .override_widget_hovered_corner_radius = Some(new_radius);
            editor_state
                .current_config
                .override_widget_active_corner_radius = Some(new_radius);
            editor_state
                .current_config
                .override_widget_open_corner_radius = Some(new_radius);
            editor_state
                .current_config
                .override_widget_noninteractive_corner_radius = Some(new_radius);
            editor_state.current_config.override_window_corner_radius = Some(new_radius);
        }
        if ui.small_button("Reset").clicked() {
            editor_state
                .current_config
                .override_widget_inactive_corner_radius = None;
            editor_state
                .current_config
                .override_widget_hovered_corner_radius = None;
            editor_state
                .current_config
                .override_widget_active_corner_radius = None;
            editor_state
                .current_config
                .override_widget_open_corner_radius = None;
            editor_state
                .current_config
                .override_widget_noninteractive_corner_radius = None;
            editor_state.current_config.override_window_corner_radius = None;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Border Width:");
        let mut width = editor_state
            .current_config
            .override_widget_inactive_bg_stroke_width
            .unwrap_or(1.0);
        if ui
            .add(egui::Slider::new(&mut width, 0.0..=5.0).text("px"))
            .changed()
        {
            editor_state
                .current_config
                .override_widget_inactive_bg_stroke_width = Some(width);
            editor_state
                .current_config
                .override_widget_hovered_bg_stroke_width = Some(width);
            editor_state
                .current_config
                .override_widget_active_bg_stroke_width = Some(width);
            editor_state
                .current_config
                .override_widget_open_bg_stroke_width = Some(width);
            editor_state
                .current_config
                .override_widget_noninteractive_bg_stroke_width = Some(width);
            editor_state.current_config.override_window_stroke_width = Some(width);
        }
        if ui.small_button("Reset").clicked() {
            editor_state
                .current_config
                .override_widget_inactive_bg_stroke_width = None;
            editor_state
                .current_config
                .override_widget_hovered_bg_stroke_width = None;
            editor_state
                .current_config
                .override_widget_active_bg_stroke_width = None;
            editor_state
                .current_config
                .override_widget_open_bg_stroke_width = None;
            editor_state
                .current_config
                .override_widget_noninteractive_bg_stroke_width = None;
            editor_state.current_config.override_window_stroke_width = None;
        }
    });
}

fn render_special_colors_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("🎨 Special Colors");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("Code background");
                ui.code("let x = 42;");
                ui.add_space(4.0);
                ui.colored_label(egui::Color32::from_rgb(255, 200, 0), "⚠ Warning message");
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "❌ Error message");
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label("Code BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_code_bg_color = visuals.code_bg_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_code_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_code_bg_color =
                        Some(editor_state.temp_code_bg_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_code_bg_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Faint BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_faint_bg_color = visuals.faint_bg_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_faint_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_faint_bg_color =
                        Some(editor_state.temp_faint_bg_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_faint_bg_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Extreme BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_extreme_bg_color = visuals.extreme_bg_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_extreme_bg_color)
                    .changed()
                {
                    editor_state.current_config.override_extreme_bg_color =
                        Some(editor_state.temp_extreme_bg_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_extreme_bg_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Warning:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_warn_fg_color = visuals.warn_fg_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_warn_fg_color)
                    .changed()
                {
                    editor_state.current_config.override_warn_fg_color =
                        Some(editor_state.temp_warn_fg_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_warn_fg_color = None;
                    editor_state.reset_temp_colors();
                }
            });

            ui.horizontal(|ui| {
                ui.label("Error:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_error_fg_color = visuals.error_fg_color;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_error_fg_color)
                    .changed()
                {
                    editor_state.current_config.override_error_fg_color =
                        Some(editor_state.temp_error_fg_color.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_error_fg_color = None;
                    editor_state.reset_temp_colors();
                }
            });
        });
    });
}

fn render_ui_options_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.heading("⚙️ UI Options");
    ui.add_space(8.0);

    ui.columns(2, |columns| {
        columns[0].vertical(|ui| {
            ui.strong("Example:");
            ui.add_space(4.0);
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("Various UI controls");
                ui.add_space(4.0);
                ui.collapsing("Collapsing", |ui| {
                    ui.label("Content");
                });
                ui.add_space(4.0);
                ui.indent("indent_example", |ui| {
                    ui.label("Indented content");
                });
            });
        });

        columns[1].vertical(|ui| {
            ui.strong("Properties:");
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                let mut button_frame = editor_state
                    .current_config
                    .override_button_frame
                    .unwrap_or(true);
                if ui.checkbox(&mut button_frame, "Button Frame").changed() {
                    editor_state.current_config.override_button_frame = Some(button_frame);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_button_frame = None;
                }
            });

            ui.horizontal(|ui| {
                let mut collapsing_header_frame = editor_state
                    .current_config
                    .override_collapsing_header_frame
                    .unwrap_or(false);
                if ui
                    .checkbox(&mut collapsing_header_frame, "Collapsing Frame")
                    .changed()
                {
                    editor_state.current_config.override_collapsing_header_frame =
                        Some(collapsing_header_frame);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_collapsing_header_frame = None;
                }
            });

            ui.horizontal(|ui| {
                let mut indent_has_left_vline = editor_state
                    .current_config
                    .override_indent_has_left_vline
                    .unwrap_or(true);
                if ui
                    .checkbox(&mut indent_has_left_vline, "Indent Vertical Line")
                    .changed()
                {
                    editor_state.current_config.override_indent_has_left_vline =
                        Some(indent_has_left_vline);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_indent_has_left_vline = None;
                }
            });

            ui.horizontal(|ui| {
                let mut striped = editor_state
                    .current_config
                    .override_striped
                    .unwrap_or(false);
                if ui.checkbox(&mut striped, "Striped Tables").changed() {
                    editor_state.current_config.override_striped = Some(striped);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_striped = None;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Resize Corner:");
                let mut size = editor_state
                    .current_config
                    .override_resize_corner_size
                    .unwrap_or(12.0);
                if ui.add(egui::Slider::new(&mut size, 6.0..=30.0)).changed() {
                    editor_state.current_config.override_resize_corner_size = Some(size);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_resize_corner_size = None;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Clip Margin:");
                let mut margin = editor_state
                    .current_config
                    .override_clip_rect_margin
                    .unwrap_or(3.0);
                if ui.add(egui::Slider::new(&mut margin, 0.0..=10.0)).changed() {
                    editor_state.current_config.override_clip_rect_margin = Some(margin);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_clip_rect_margin = None;
                }
            });

            ui.horizontal(|ui| {
                ui.label("Popup Shadow:");
                let popup_shadow = editor_state
                    .current_config
                    .override_popup_shadow_size
                    .unwrap_or(16);
                let mut popup_shadow_f32 = popup_shadow as f32;
                if ui
                    .add(egui::Slider::new(&mut popup_shadow_f32, 0.0..=50.0))
                    .changed()
                {
                    editor_state.current_config.override_popup_shadow_size =
                        Some(popup_shadow_f32 as u8);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_popup_shadow_size = None;
                }
            });
        });
    });
}

fn render_advanced_widget_states_section(ui: &mut egui::Ui, editor_state: &mut ThemeEditorState) {
    ui.collapsing("🔧 Advanced Per-State Controls", |ui| {
        ui.label(
            "Fine-tune weak fills, per-state radii/widths, and expansion for each widget state",
        );
        ui.add_space(8.0);

        ui.collapsing("Inactive State (Advanced)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Weak BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_inactive_weak_bg_fill =
                    visuals.widgets.inactive.weak_bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_inactive_weak_bg_fill)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_inactive_weak_bg_fill =
                        Some(editor_state.temp_widget_inactive_weak_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_inactive_weak_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });
            ui.horizontal(|ui| {
                ui.label("FG Stroke Width:");
                let mut width = editor_state
                    .current_config
                    .override_widget_inactive_fg_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state
                        .current_config
                        .override_widget_inactive_fg_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_inactive_fg_stroke_width = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Expansion:");
                let mut expansion = editor_state
                    .current_config
                    .override_widget_inactive_expansion
                    .unwrap_or(0.0);
                if ui
                    .add(egui::Slider::new(&mut expansion, -5.0..=10.0))
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_inactive_expansion = Some(expansion);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_inactive_expansion = None;
                }
            });
        });

        ui.collapsing("Hovered State (Advanced)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Weak BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_hovered_weak_bg_fill =
                    visuals.widgets.hovered.weak_bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_hovered_weak_bg_fill)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_hovered_weak_bg_fill =
                        Some(editor_state.temp_widget_hovered_weak_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_hovered_weak_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });
            ui.horizontal(|ui| {
                ui.label("FG Stroke Width:");
                let mut width = editor_state
                    .current_config
                    .override_widget_hovered_fg_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state
                        .current_config
                        .override_widget_hovered_fg_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_hovered_fg_stroke_width = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Expansion:");
                let mut expansion = editor_state
                    .current_config
                    .override_widget_hovered_expansion
                    .unwrap_or(0.0);
                if ui
                    .add(egui::Slider::new(&mut expansion, -5.0..=10.0))
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_hovered_expansion = Some(expansion);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_hovered_expansion = None;
                }
            });
        });

        ui.collapsing("Active State (Advanced)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Weak BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_active_weak_bg_fill = visuals.widgets.active.weak_bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_active_weak_bg_fill)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_active_weak_bg_fill =
                        Some(editor_state.temp_widget_active_weak_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_active_weak_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });
            ui.horizontal(|ui| {
                ui.label("FG Stroke Width:");
                let mut width = editor_state
                    .current_config
                    .override_widget_active_fg_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state
                        .current_config
                        .override_widget_active_fg_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_active_fg_stroke_width = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Expansion:");
                let mut expansion = editor_state
                    .current_config
                    .override_widget_active_expansion
                    .unwrap_or(0.0);
                if ui
                    .add(egui::Slider::new(&mut expansion, -5.0..=10.0))
                    .changed()
                {
                    editor_state.current_config.override_widget_active_expansion = Some(expansion);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_widget_active_expansion = None;
                }
            });
        });

        ui.collapsing("Open State (Advanced)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Weak BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_open_weak_bg_fill = visuals.widgets.open.weak_bg_fill;
                if ui
                    .color_edit_button_srgba(&mut editor_state.temp_widget_open_weak_bg_fill)
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_open_weak_bg_fill =
                        Some(editor_state.temp_widget_open_weak_bg_fill.to_array());
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_open_weak_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });
            ui.horizontal(|ui| {
                ui.label("FG Stroke Width:");
                let mut width = editor_state
                    .current_config
                    .override_widget_open_fg_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state
                        .current_config
                        .override_widget_open_fg_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_open_fg_stroke_width = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Expansion:");
                let mut expansion = editor_state
                    .current_config
                    .override_widget_open_expansion
                    .unwrap_or(0.0);
                if ui
                    .add(egui::Slider::new(&mut expansion, -5.0..=10.0))
                    .changed()
                {
                    editor_state.current_config.override_widget_open_expansion = Some(expansion);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state.current_config.override_widget_open_expansion = None;
                }
            });
        });

        ui.collapsing("Noninteractive State (Advanced)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Weak BG:");
                let visuals = editor_state.current_config.to_visuals();
                editor_state.temp_widget_noninteractive_weak_bg_fill =
                    visuals.widgets.noninteractive.weak_bg_fill;
                if ui
                    .color_edit_button_srgba(
                        &mut editor_state.temp_widget_noninteractive_weak_bg_fill,
                    )
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_weak_bg_fill = Some(
                        editor_state
                            .temp_widget_noninteractive_weak_bg_fill
                            .to_array(),
                    );
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_weak_bg_fill = None;
                    editor_state.reset_temp_colors();
                }
            });
            ui.horizontal(|ui| {
                ui.label("FG Stroke Width:");
                let mut width = editor_state
                    .current_config
                    .override_widget_noninteractive_fg_stroke_width
                    .unwrap_or(1.0);
                if ui.add(egui::Slider::new(&mut width, 0.0..=5.0)).changed() {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_fg_stroke_width = Some(width);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_fg_stroke_width = None;
                }
            });
            ui.horizontal(|ui| {
                ui.label("Expansion:");
                let mut expansion = editor_state
                    .current_config
                    .override_widget_noninteractive_expansion
                    .unwrap_or(0.0);
                if ui
                    .add(egui::Slider::new(&mut expansion, -5.0..=10.0))
                    .changed()
                {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_expansion = Some(expansion);
                }
                if ui.small_button("Reset").clicked() {
                    editor_state
                        .current_config
                        .override_widget_noninteractive_expansion = None;
                }
            });
        });
    });
}
