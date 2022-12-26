//! GUI related functionality.
//!
//! This module contains all relm4 components and widgets used in the launcher.
//!
//! Main component is [`AppModel`].
//!
//! [`AppModel`]: app/struct.AppModel.html

pub mod app;
pub mod components;

pub use app::AppModel;

/// Application CSS.
pub const CSS: &str = include_str!("../../resources/style.css");
