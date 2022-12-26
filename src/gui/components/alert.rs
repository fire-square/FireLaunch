//! Alert dialog component.
//!
//! This component is used to show a dialog with a message and two/three buttons.

use gtk::prelude::*;
use relm4::prelude::*;

/// Configuration for the alert dialog component
pub struct AlertSettings {
	/// Large text
	pub text: String,
	/// Optional secondary, smaller text
	pub secondary_text: Option<String>,
	/// Modal dialogs freeze other windows as long they are visible
	pub is_modal: bool,
	/// Sets color of the accept button to red if the theme supports it
	pub destructive_accept: bool,
	/// Label of the confirm button. Type of the button is [`gtk::ResponseType::Ok`].
	pub confirm_label: String,
	/// Label of the cancel button. If [`None`], the button is not created. Type of the button is [`gtk::ResponseType::Cancel`].
	pub cancel_label: Option<String>,
	/// Label of the option button. If [`None`], the button is not created. Type of the button is [`gtk::ResponseType::Other(0)`].
	pub option_label: Option<String>,
	/// Alert type
	pub alert_type: gtk::MessageType,
}

/// Alert dialog component.
pub struct Alert {
	settings: AlertSettings,
	is_active: bool,
}

/// Messages that can be sent to the alert dialog component
#[derive(Debug)]
pub enum AlertMsg {
	/// Message sent by the parent to view the dialog
	Show,
	#[doc(hidden)]
	Response(gtk::ResponseType),
}

/// User action performed on the alert dialog.
#[derive(Debug)]
pub enum AlertResponse {
	/// User clicked the first button
	Confirm,
	/// User clicked the second button
	Cancel,
	/// User clicked the third button
	Option,
}

/// Widgets of the alert dialog component.
#[relm4::component(pub)]
impl SimpleComponent for Alert {
	type Widgets = AlertWidgets;
	type Init = AlertSettings;
	type Input = AlertMsg;
	type Output = AlertResponse;

	view! {
		#[name = "dialog"]
		gtk::MessageDialog {
			set_message_type: model.settings.alert_type,
			#[watch]
			set_visible: model.is_active,
			connect_response[sender] => move |_, response| {
				sender.input(AlertMsg::Response(response));
			},

			// Apply configuration
			set_text: Some(&model.settings.text),
			set_secondary_text: model.settings.secondary_text.as_deref(),
			set_modal: model.settings.is_modal,
			add_button: (&model.settings.confirm_label, gtk::ResponseType::Ok),
		}
	}

	fn init(
		settings: AlertSettings,
		root: &Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Alert {
			settings,
			is_active: false,
		};

		let widgets = view_output!();

		if let Some(btn2_label) = &model.settings.cancel_label {
			widgets
				.dialog
				.add_button(btn2_label, gtk::ResponseType::Cancel);
		}

		if let Some(btn3_label) = &model.settings.option_label {
			widgets
				.dialog
				.add_button(btn3_label, gtk::ResponseType::Other(0));
		}

		if model.settings.destructive_accept {
			let accept_widget = widgets
				.dialog
				.widget_for_response(gtk::ResponseType::Ok)
				.expect("No button for accept response set");
			accept_widget.add_css_class("destructive-action");
		}

		ComponentParts { model, widgets }
	}

	fn update(&mut self, input: AlertMsg, sender: ComponentSender<Self>) {
		match input {
			AlertMsg::Show => {
				self.is_active = true;
			}
			AlertMsg::Response(ty) => {
				self.is_active = false;
				let _ = sender.output(match ty {
					gtk::ResponseType::Ok => AlertResponse::Confirm,
					gtk::ResponseType::Other(0) => AlertResponse::Option,
					_ => AlertResponse::Cancel,
				});
			}
		}
	}
}
