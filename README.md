# egui-thematic

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/egui--thematic-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/egui-thematic)
[<img alt="crates.io" src="https://img.shields.io/crates/v/egui-thematic.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/egui-thematic)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-egui--thematic-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/egui-thematic)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/matthewjberger/egui-thematic/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/matthewjberger/egui-thematic/actions?query=branch%3Amain)

A comprehensive theme editor and configuration system for [egui](https://github.com/emilk/egui) applications with live preview, preset management, random theme generation, and persistence.

## Features

- **Full Theme Configuration**: Customize all visual aspects of your egui application
  - Text colors (normal and weak)
  - Window and panel fills
  - Selection backgrounds
  - Hyperlink colors
  - Background colors (faint, extreme, code)
  - Warning and error colors

- **Built-in Presets**: Dark and Light themes included out of the box

- **Random Theme Generation**: Generate completely random themes with a single click for exploration and inspiration

- **Live Preview**: See changes in real-time as you edit with an interactive preview panel

- **Persistence**: Save and load themes to/from JSON files for easy sharing and reuse

- **Interactive Theme Editor**: Full-featured UI with:
  - Collapsible sections for organized editing
  - Color pickers for all theme elements
  - Individual reset buttons to restore defaults
  - Preview panel showing all UI elements in real-time

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
egui-thematic = "0.1.0" # supports egui 0.31.1
```

## Usage

### Basic Usage with eframe

```rust
use egui_thematic::{ThemeConfig, ThemeEditorState, render_theme_editor};

struct MyApp {
    theme_editor_state: ThemeEditorState,
    show_theme_editor: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            theme_editor_state: ThemeEditorState::default(),
            show_theme_editor: true,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply the current theme
        let visuals = self.theme_editor_state.current_config.to_visuals();
        ctx.set_visuals(visuals);

        // Show the theme editor window
        if self.show_theme_editor {
            egui::Window::new("Theme Editor")
                .open(&mut self.show_theme_editor)
                .show(ctx, |ui| {
                    render_theme_editor(ui, &mut self.theme_editor_state);
                });
        }

        // Your app UI here
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Application");
            if ui.button("Toggle Theme Editor").clicked() {
                self.show_theme_editor = !self.show_theme_editor;
            }
        });
    }
}
```

### Integration with Bevy

```rust
use bevy::prelude::*;
use egui_thematic::{ThemeEditorState, render_theme_editor};

fn setup(mut commands: Commands) {
    commands.init_resource::<ThemeEditorState>();
}

fn ui_system(
    mut theme_editor_state: ResMut<ThemeEditorState>,
    mut egui_contexts: Query<&mut EguiContext>,
) {
    for mut egui_context in egui_contexts.iter_mut() {
        let context = egui_context.get_mut();

        // Apply the current theme
        let visuals = theme_editor_state.current_config.to_visuals();
        context.set_visuals(visuals);

        // Render the theme editor
        egui::Window::new("Theme Editor")
            .show(context, |ui| {
                render_theme_editor(ui, &mut theme_editor_state);
            });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .run();
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

The randomize feature is great for:
- Quickly exploring different color combinations
- Finding inspiration for custom themes
- Testing your UI with extreme color variations
- Having fun with wild color schemes

## Demo Application

A comprehensive demo application is included that showcases all features of egui-thematic.

### Running the Demo

Using `just` (recommended):
```bash
just run
```

Or directly with cargo:
```bash
cargo run --release --manifest-path ./demo/Cargo.toml
```

The demo provides:
- A fully functional theme editor with all features
- Sample UI elements showing how the theme affects different widgets
- Quick preset buttons (Dark, Light, Randomize)
- Interactive instructions and examples

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

The `ThemeConfig` struct provides fine-grained control over all visual aspects:

```rust
pub struct ThemeConfig {
    pub name: String,
    pub dark_mode: bool,

    // Optional color overrides (None = use egui defaults)
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
```

All color overrides are optional - when `None`, egui's default values for the selected mode (dark/light) are used.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) by Emil Ernerfeldt
- Inspired by the need for easy theme customization in egui applications
