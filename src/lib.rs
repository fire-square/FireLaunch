//! # Firesquare Launcher
//!
//! This is open source launcher for Minecraft game. It supports
//! both vanilla and modded versions of the game.
//!
//! It is written with relm4 and uses GTK4 for UI.
//!
//! **Currently it is in early development stage.**
//!
//! ## Features
//!
//! - [ ] Launching vanilla Minecraft
//! - [ ] Launching modded Minecraft
//! - [ ] Logging in to Mojang account
//! - [ ] Logging in to Microsoft account
//! - [ ] Logging in to Firesquare account
//! - [ ] Deb, RPM, Flatpak, AppImage, Snap, MSI packages

#![doc(html_root_url = "https://docs.rs/firesquare-launcher/0.1.0")]
#![warn(missing_docs)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate relm4;
// #[macro_use]
// extern crate tracker;

pub mod gui;
pub mod utils;

/// Name of the application.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Version of the application.
pub const NAME: &str = env!("CARGO_PKG_NAME");
