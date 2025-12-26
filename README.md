# egui-thematic

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/egui--thematic-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/egui-thematic)
[<img alt="crates.io" src="https://img.shields.io/crates/v/egui-thematic.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/egui-thematic)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-egui--thematic-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/egui-thematic)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/matthewjberger/egui-thematic/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/matthewjberger/egui-thematic/actions?query=branch%3Amain)

A theme editor and configuration system for [egui](https://github.com/emilk/egui) applications with live preview, preset management, random theme generation, and persistence.

<img width="1280" height="1109" alt="image" src="https://github.com/user-attachments/assets/c2d8b6ab-5d45-4979-b3d1-b375b4190236" />

## Features

- **Complete Theme Configuration**: Full control over all 73 egui visual properties
  - 8 core color palette entries (background, surface, primary, text, text weak, widget fill/hover/active)
  - 3 global appearance controls (corner radius, border width, window shadow)
  - 5 widget states × 8 properties each (noninteractive, inactive, hovered, active, open)
  - Additional UI controls (text cursor, resize corners, button frames, etc.)

- **Two-Level Interface**:
  - **Simple Mode**: 8 colors + 3 sliders for quick theming (covers 80% of use cases)
  - **Advanced Mode**: Collapsible sections for fine-grained per-widget-state control

- **Live Preview Window**: Separate window shows changes instantly with Apply/Revert buttons
  - See all changes before committing
  - Keyboard shortcuts: Ctrl+Enter to apply, Escape to revert
  - Fully interactive preview with all widget types

- **Smart Global Controls**: Automatically detect when widget states have mixed values
  - Disabled when states differ to prevent accidental overwrites
  - Shows "(mixed)" indicator for clarity

- **9 Built-in Presets**: Dark, Light, Dracula, Nord, Gruvbox, Solarized (Dark/Light), Monokai, One Dark, Tokyo Night, Catppuccin Mocha

- **Random Theme Generation**: Generate completely random themes with a single click for exploration and inspiration

- **Code Export**: Export themes as JSON or Rust code for easy integration

- **Persistence**: Save and load themes to/from JSON files for easy sharing and reuse

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui-thematic = "0.1.1" # supports egui 0.33.0
```

## Usage

### Quickstart

Add egui-thematic to your egui application, here's an example using the [nightshade](https://github.com/matthewjberger/nightshade) game engine:

**Cargo.toml:**

```toml
[dependencies]
nightshade = "0.6.7"
egui-thematic = "0.1.1"
```

**main.rs:**

```rust
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

    fn initialize(&mut self, world: &mut World) {
        world.resources.user_interface.enabled = true;
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
                if ui.button("Toggle Theme Editor").clicked() {
                    self.show_theme_editor = !self.show_theme_editor;
                }
            });
        });
    }

    fn on_keyboard_input(&mut self, world: &mut World, key_code: KeyCode, key_state: KeyState) {
        if matches!((key_code, key_state), (KeyCode::KeyQ, KeyState::Pressed)) {
            world.resources.window.should_exit = true;
        }
    }
}

```

### Working with Themes

#### Using Presets

```rust
use egui_thematic::ThemeConfig;

// Use built-in presets
let dark_theme = ThemeConfig::dark_preset();
let light_theme = ThemeConfig::light_preset();

// Apply to egui context
ctx.set_visuals(dark_theme.to_visuals());
```

#### Saving and Loading Themes

```rust
use egui_thematic::ThemeConfig;
use std::path::Path;

// Save theme to file
let theme = ThemeConfig::dark_preset();
theme.save_to_file(Path::new("my_theme.theme.json"))?;

// Load theme from file
let loaded_theme = ThemeConfig::load_from_file(Path::new("my_theme.theme.json"))?;
```

#### Generating Random Themes

```rust
use egui_thematic::ThemeConfig;

// Generate a completely random theme
let random_theme = ThemeConfig::randomize();

// Apply it
ctx.set_visuals(random_theme.to_visuals());
```

The randomize feature can be used for:

- Exploring different color combinations
- Finding inspiration for custom themes
- Testing your UI with extreme color variations

## Demo Application

A demo application is included that showcases the features of egui-thematic.

### Running the Demo

Using `just`:

```bash
just run
```

Or directly with cargo:

```bash
cargo run --release --manifest-path ./demo/Cargo.toml
```

The demo includes:

- Theme editor with all features
- Sample UI elements showing how the theme affects different widgets
- Preset buttons (Dark, Light, Randomize)
- Instructions and examples

## Development

### Prerequisites

- Rust 1.90.0 or later
- [just](https://github.com/casey/just) (optional, for convenience commands)

### Available Commands

Using `justfile` for convenience:

- `just run` - Run the demo application in release mode
- `just build` - Build the crate in release mode
- `just check` - Run cargo check and format verification
- `just format` - Format all code with rustfmt
- `just fix` - Run clippy with auto-fixes
- `just lint` - Run clippy with warnings as errors
- `just test` - Run all tests
- `just versions` - Display tool versions

## Theme Configuration

The `ThemeConfig` struct provides complete control over all 73 egui visual properties:

```rust
pub struct ThemeConfig {
    pub name: String,
    pub dark_mode: bool,

    // Text colors (2 properties)
    pub override_text_color: Option<[u8; 4]>,
    pub override_weak_text_color: Option<[u8; 4]>,

    // Windows (5 properties)
    pub override_window_fill: Option<[u8; 4]>,
    pub override_window_stroke_color: Option<[u8; 4]>,
    pub override_window_stroke_width: Option<f32>,
    pub override_window_corner_radius: Option<u8>,
    pub override_window_shadow_size: Option<u8>,

    // Panels (1 property)
    pub override_panel_fill: Option<[u8; 4]>,

    // Selection (3 properties)
    pub override_selection_bg: Option<[u8; 4]>,
    pub override_selection_stroke_color: Option<[u8; 4]>,
    pub override_selection_stroke_width: Option<f32>,

    // Widget States (5 states × 8 properties = 40 properties)
    // Each state has: bg_fill, weak_bg_fill, bg_stroke (color + width),
    // corner_radius, fg_stroke (color + width), expansion
    pub override_widget_noninteractive_*: Option<...>,
    pub override_widget_inactive_*: Option<...>,
    pub override_widget_hovered_*: Option<...>,
    pub override_widget_active_*: Option<...>,
    pub override_widget_open_*: Option<...>,

    // UI Controls (8 properties)
    pub override_resize_corner_size: Option<f32>,
    pub override_text_cursor_width: Option<f32>,
    pub override_clip_rect_margin: Option<f32>,
    pub override_button_frame: Option<bool>,
    pub override_collapsing_header_frame: Option<bool>,
    pub override_indent_has_left_vline: Option<bool>,
    pub override_striped: Option<bool>,
    pub override_slider_trailing_fill: Option<bool>,
}
```

All overrides are optional - when `None`, egui's default values for the selected mode (dark/light) are used. This allows for:
- Minimal configuration (just override what you need)
- Full control (override all 73 properties for complete customization)
- Mix and match (use defaults for most things, customize specific details)

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
