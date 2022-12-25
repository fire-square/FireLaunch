use gtk::{
	prelude::{BoxExt, ButtonExt, OrientableExt},
	traits::GtkWindowExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

#[macro_use]
extern crate log;
#[macro_use]
extern crate relm4;
// #[macro_use]
// extern crate tracker;

struct AppModel;

#[derive(Debug)]
enum AppInput {
	LaunchMinecraft,
}

#[component]
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

	// Initialize the UI.
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

fn main() {
	if std::env::var("RUST_LOG").is_err() {
		std::env::set_var("RUST_LOG", "info");
	}
	env_logger::init();
	log_panics::init();

	// Get package info
	const VERSION: &str = env!("CARGO_PKG_VERSION");
	const NAME: &str = env!("CARGO_PKG_NAME");

	// Load the CSS file
	const CSS: &str = include_str!("../style.css");

	info!("Running {} {}", NAME, VERSION);

	let app = RelmApp::new("xyz.frsqr.launcher");
	relm4::set_global_css(CSS);
	app.run::<AppModel>(());
}
