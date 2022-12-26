//! AppModel is the main component of the application.
//!
//! It contains the main window and all other components. It is responsible for
//! handling all user input and launching the game.

use super::components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use super::CSS;
use crate::utils::net::NetClient;
use gtk::{
	prelude::{BoxExt, ButtonExt, OrientableExt},
	traits::GtkWindowExt,
};
use relm4::{
	gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
	RelmWidgetExt, SimpleComponent,
};

/// Shared application state.
pub struct SharedState {
	/// Network client.
	pub net_client: NetClient,
}

/// AppModel state.
pub struct AppModel {
	// state: Arc<SharedState>,
	dialog: Controller<Alert>,
}

/// AppModel commands.
#[derive(Debug)]
pub enum AppMsg {
	/// Launch minecraft.
	LaunchMinecraft,
	/// Force cofob to work.
	ForceCofob,
	/// Ignore.
	Ignore,
}

/// AppModel component implementation.
#[component(pub)]
impl SimpleComponent for AppModel {
	type Widgets = AppWidgets;
	type Init = SharedState;
	type Input = AppMsg;
	type Output = ();

	view! {
		#[root]
		gtk::ApplicationWindow {
			set_title: Some("FireLaunch"),
			set_icon_name: Some("firesquare-launcher"),
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_spacing: 5,
				set_margin_all: 10,
				gtk::Button {
					set_label: "Запустить minecraft",
					connect_clicked[sender] => move |_| {
						sender.input(AppMsg::LaunchMinecraft)
					}
				},
				append = &gtk::Button {
					set_label: "Заставить кофоба работать",
					connect_clicked[sender] => move |_| {
						sender.input(AppMsg::ForceCofob)
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
		let model = AppModel {
			// state: Arc::new(params),
			dialog: Alert::builder()
				.transient_for(root)
				.launch(AlertSettings {
					text: String::from("Кофоба невозможно заставить работать"),
					secondary_text: Some(String::from("И че ты мне сделаешь?)")),
					confirm_label: String::from("Ничего"),
					cancel_label: String::from("Ничего"),
					option_label: None,
					is_modal: true,
					destructive_accept: true,
				})
				.forward(sender.input_sender(), convert_alert_response),
		};

		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppMsg::LaunchMinecraft => {
				info!("Launching minecraft");
				todo!("Launch minecraft")
			}
			AppMsg::ForceCofob => {
				self.dialog.emit(AlertMsg::Show);
			}
			AppMsg::Ignore => {}
		}
	}
}

fn convert_alert_response(response: AlertResponse) -> AppMsg {
	match response {
		AlertResponse::Confirm => AppMsg::Ignore,
		AlertResponse::Cancel => AppMsg::Ignore,
		AlertResponse::Option => AppMsg::Ignore,
	}
}

impl AppModel {
	/// Launch application.
	///
	/// This function is called from `main.rs`.
	pub fn launch() {
		let net_client = NetClient::new();
		let state = SharedState { net_client };

		let app = RelmApp::new("xyz.frsqr.launcher");
		relm4::set_global_css(CSS);
		app.run::<AppModel>(state);
	}
}
