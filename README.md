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
egui-thematic = "0.1.0" # supports egui 0.33.0
```

## Usage

### Quickstart

Add egui-thematic to your nightshade application:

**Cargo.toml:**
```toml
[dependencies]
nightshade = { git = "https://github.com/matthewjberger/nightshade" }
egui-thematic = "0.1.0"
```

**main.rs:**
```rust
use egui_thematic::{render_theme_panel, ThemeEditorState};
use nightshade::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    launch(App::default())?;
    Ok(())
}

#[derive(Default)]
struct App {
    theme_editor_state: ThemeEditorState,
    show_theme_editor: bool,
}

impl State for App {
    fn title(&self) -> &str {
        "My App"
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
        // Render the theme panel - handles theme application and editor window
        render_theme_panel(ui_context, &mut self.theme_editor_state, &mut self.show_theme_editor);

        // Your app UI here
        egui::CentralPanel::default().show(ui_context, |ui| {
            ui.heading("My Application");
            if ui.button("Toggle Theme Editor").clicked() {
                self.show_theme_editor = !self.show_theme_editor;
            }
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
