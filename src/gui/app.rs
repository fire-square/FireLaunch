//! AppModel is the main component of the application.
//!
//! It contains the main window and all other components. It is responsible for
//! handling all user input and launching the game.

use super::async_worker::{AsyncWorkerModel, AsyncWorkerMsg};
use super::components::alert::{Alert, AlertMsg, AlertResponse, AlertSettings};
use super::CSS;
use gtk::{prelude::*, traits::GtkWindowExt};
use relm4::{
	gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmApp,
	RelmWidgetExt, SimpleComponent, WorkerController,
};
use std::convert::identity;

/// AppModel state.
pub struct AppModel {
	force_cofob_dialog: Controller<Alert>,
	internet_unavailable_dialog: Controller<Alert>,
	async_worker: WorkerController<AsyncWorkerModel>,
	app_window: gtk::ApplicationWindow,
	progress_bar: gtk::ProgressBar,
}

/// AppModel commands.
#[derive(Debug)]
pub enum AppMsg {
	/// Launch minecraft.
	LaunchMinecraft,
	/// Set progress bar fraction.
	SetProgressBarFraction(f64),
	/// Set progress bar text.
	SetProgressBarText(Option<String>),
	/// Hide progress bar.
	HideProgressBar,
	/// Show progress bar.
	ShowProgressBar,
	/// Force cofob to work.
	ForceCofob,
	/// Internet is unavailable.
	InternetUnavailable,
	/// Close application.
	CloseApp,
	/// Ignore.
	Ignore,
}

/// AppModel component implementation.
#[component(pub)]
impl SimpleComponent for AppModel {
	type Widgets = AppWidgets;
	type Init = ();
	type Input = AppMsg;
	type Output = ();

	view! {
		#[root]
		gtk::ApplicationWindow {
			set_title: Some("FireLaunch"),
			set_icon_name: Some("firelaunch"),
			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_valign: gtk::Align::Fill,

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

					gtk::Button {
						set_label: "Заставить кофоба работать",
						connect_clicked[sender] => move |_| {
							sender.input(AppMsg::ForceCofob)
						}
					},

					#[name = "progress_bar"]
					gtk::ProgressBar {
						set_fraction: 0.0,
						set_show_text: true,
					},
				},

				// TODO: this should be at the very bottom
				gtk::Label {
					set_label: &format!("Version: {}-{}", env!("VERGEN_GIT_SEMVER"), env!("VERGEN_GIT_SHA_SHORT")),
				}
			}
		}
	}

	fn init(
		_params: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let widgets = view_output!();

		let model = AppModel {
			force_cofob_dialog: Alert::builder()
				.transient_for(root)
				.launch(AlertSettings {
					text: String::from("Кофоба невозможно заставить работать"),
					secondary_text: Some(String::from("И че ты мне сделаешь?)")),
					confirm_label: String::from("Ничего"),
					cancel_label: None,
					option_label: None,
					is_modal: true,
					destructive_accept: true,
					alert_type: gtk::MessageType::Info,
				})
				.forward(sender.input_sender(), convert_ignore_alert_response),
			internet_unavailable_dialog: Alert::builder()
				.transient_for(root)
				.launch(AlertSettings {
					text: String::from("Интернет недоступен"),
					secondary_text: Some(String::from(
						"Часть функций может работать некорректно. Пожалуйста, проверьте подключение к интернету.",
					)),
					confirm_label: String::from("Закрыть"),
					cancel_label: None,
					option_label: None,
					is_modal: true,
					destructive_accept: true,
					alert_type: gtk::MessageType::Error,
				})
				.forward(
					sender.input_sender(),
					convert_ignore_alert_response,
				),
			async_worker: AsyncWorkerModel::builder()
				.detach_worker(())
				.forward(sender.input_sender(), identity),
			app_window: root.clone(),
			progress_bar: widgets.progress_bar.clone(),
		};

		model.async_worker.emit(AsyncWorkerMsg::CheckConnection);

		widgets.progress_bar.hide();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
		match message {
			AppMsg::LaunchMinecraft => {
				info!("Launching minecraft");
				self.async_worker.emit(AsyncWorkerMsg::DownloadAssets);
			}
			AppMsg::SetProgressBarFraction(fraction) => {
				self.progress_bar.set_fraction(fraction);
			}
			AppMsg::SetProgressBarText(text) => match text {
				Some(text) => {
					self.progress_bar.set_text(Some(&text));
					self.progress_bar.set_show_text(true);
				}
				None => {
					self.progress_bar.set_text(None);
					self.progress_bar.set_show_text(false);
				}
			},
			AppMsg::HideProgressBar => {
				self.progress_bar.set_fraction(0.0);
				self.progress_bar.hide();
			}
			AppMsg::ShowProgressBar => {
				self.progress_bar.set_fraction(0.0);
				self.progress_bar.show();
			}
			AppMsg::ForceCofob => {
				self.force_cofob_dialog.emit(AlertMsg::Show);
			}
			AppMsg::InternetUnavailable => {
				self.internet_unavailable_dialog.emit(AlertMsg::Show);
			}
			AppMsg::CloseApp => {
				info!("Closing app");
				self.app_window.close();
			}
			AppMsg::Ignore => {}
		}
	}
}

fn convert_ignore_alert_response(response: AlertResponse) -> AppMsg {
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
		let app = RelmApp::new("xyz.frsqr.launcher");
		relm4::set_global_css(CSS);
		app.run::<AppModel>(());
	}
}
