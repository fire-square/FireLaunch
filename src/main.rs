use gtk::{
	prelude::{BoxExt, ButtonExt, GtkWindowExt},
	traits::OrientableExt,
};
use relm4::{gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent};

struct AppModel {
	counter: i64,
}

#[derive(Debug)]
enum AppInput {
	Increment,
	Decrement,
}

#[relm4::component]
impl SimpleComponent for AppModel {
	type Widgets = AppWidgets;

	type Init = i64;

	type Input = AppInput;
	type Output = ();

	view! {
		gtk::Window {
			set_title: Some("firesquare launcher"),
			set_default_width: 300,
			set_default_height: 100,

			gtk::Box {
				set_orientation: gtk::Orientation::Vertical,
				set_spacing: 5,
				set_margin_all: 5,

				gtk::Button::with_label("Increment") {
					connect_clicked[sender] => move |_| {
						sender.input(AppInput::Increment);
					}
				},

				gtk::Button::with_label("Decrement") {
					connect_clicked[sender] => move |_| {
						sender.input(AppInput::Decrement);
					}
				},

				gtk::Label {
					#[watch]
					set_label: &format!("Counter: {}", model.counter),
					set_margin_all: 5,
				}
			}
		}
	}

	// Initialize the UI.
	fn init(
		counter: Self::Init,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = AppModel { counter };

		// Insert the macro code generation here
		let widgets = view_output!();

		ComponentParts { model, widgets }
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			AppInput::Increment => {
				self.counter = self.counter.wrapping_add(1);
			}
			AppInput::Decrement => {
				self.counter = self.counter.wrapping_sub(1);
			}
		}
	}
}

fn main() {
	let app = RelmApp::new("xyz.frsqr.launcher");
	app.run::<AppModel>(100);
}
