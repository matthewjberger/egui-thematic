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
//! ```rust
//! use egui_thematic::{ThemeConfig, ThemeEditorState, render_theme_panel};
//!
//! let mut theme_editor_state = ThemeEditorState::default();
//! let mut show_theme_editor = true;
//!
//! // In your UI code:
//! // render_theme_panel(ctx, &mut theme_editor_state, &mut show_theme_editor);
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

mod config;
mod state;
mod ui;

pub use config::ThemeConfig;
pub use state::ThemeEditorState;
pub use ui::{render_theme_editor, render_theme_panel};
