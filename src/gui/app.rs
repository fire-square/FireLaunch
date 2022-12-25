//! AppModel is the main component of the application.
//!
//! It contains the main window and all other components. It is responsible for
//! handling all user input and launching the game.

use super::CSS;
use gtk::{
	prelude::{BoxExt, ButtonExt, OrientableExt},
	traits::GtkWindowExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

/// AppModel state.
///
/// Empty for now.
pub struct AppModel;

/// AppModel commands.
#[derive(Debug)]
pub enum AppInput {
	/// Launch minecraft.
	LaunchMinecraft,
}

/// AppModel component implementation.
#[component(pub)]
impl SimpleComponent for AppModel {
	type Widgets = AppWidgets;
	type Init = ();
	type Input = AppInput;
	type Output = ();

	view! {
		#[root]
		gtk::ApplicationWindow {
			set_title: Some("FireLaunch"),
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_spacing: 5,
				set_margin_all: 10,
				gtk::Button {
					set_label: "Запустить minecraft",
					connect_clicked[sender] => move |_| {
						sender.input(AppInput::LaunchMinecraft)
					}
				},
				append = &gtk::Button {
					set_label: "Заставить кофоба работать",
					connect_clicked[sender] => move |_| {
						sender.input(AppInput::LaunchMinecraft)
					}
				},
			}
		}
	}

	fn init(
		_params: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = AppModel {};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppInput::LaunchMinecraft => {
				info!("Launching minecraft");
				todo!("Launch minecraft")
			}
		}
	}
}

impl AppModel {
	/// Launch application.
	///
	/// This function is called from `main.rs`.
	pub fn launch() {
		let app = RelmApp::new("xyz.frsqr.launcher");
		relm4::set_global_css(CSS);
		app.run::<AppModel>(());
	}
}
