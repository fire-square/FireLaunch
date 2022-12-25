//! GUI related functionality.
//!
//! This module contains all relm4 components and widgets used in the launcher.
//!
//! Main component is [`AppModel`].
//!
//! [`AppModel`]: struct.AppModel.html

pub mod app;

/// Re-export of [`AppModel`].
pub use app::AppModel;

/// Application CSS.
const CSS: &str = include_str!("../../style.css");
